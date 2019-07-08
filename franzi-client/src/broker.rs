//! Kafka client
//! Note: This needs better type tetris, I'm really not happy how this is working out.

use byteorder::ByteOrder;
use bytes::{BufMut, Bytes, BytesMut};
use franzi_base::types::KafkaString;
use franzi_base::{Error, FromKafkaBytes, KafkaRequest, ToKafkaBytes};
use franzi_proto::header::RequestHeader;
use futures::sync::{mpsc, oneshot};
use futures::{future, try_ready, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use tokio::codec::{Decoder, Encoder, Framed};
use tokio::net::tcp::TcpStream;
use tokio::prelude::stream::{SplitSink, SplitStream};

// TODO: Find something more efficient?
pub type Correlation = (i32, oneshot::Sender<Bytes>);

pub type BrokerConnection<St, Et, Si, Ei> = (BrokerClient<Si, Ei>, BrokerResponses<St, Et>);

pub type BrokerTcpConnection = BrokerConnection<
    SplitStream<Framed<TcpStream, ConnectionCodec>>,
    std::io::Error,
    SplitSink<Framed<TcpStream, ConnectionCodec>>,
    std::io::Error,
>;

pub struct BrokerClient<Si, E>
where
    Si: futures::Sink<SinkItem = Bytes, SinkError = E>,
{
    sink: Si,
    correlations_tx: mpsc::UnboundedSender<Correlation>,
}

pub struct BrokerResponses<St, E>
where
    St: futures::Stream<Item = BytesMut, Error = E>,
{
    stream: St,
    correlations_rx: mpsc::UnboundedReceiver<Correlation>,
    correlations: HashMap<i32, oneshot::Sender<Bytes>>,
    correlations_closed: bool,
}

pub fn connect(
    addr: &SocketAddr,
) -> impl Future<Item = BrokerTcpConnection, Error = std::io::Error> {
    TcpStream::connect(addr).map(new_broker_connection)
}

pub fn new_broker_connection(sock: TcpStream) -> BrokerTcpConnection {
    let correlations = mpsc::unbounded();
    let (sink, stream) = Framed::new(sock, ConnectionCodec()).split();
    (
        BrokerClient {
            sink,
            correlations_tx: correlations.0,
        },
        BrokerResponses {
            stream,
            correlations_rx: correlations.1,
            correlations: Default::default(),
            correlations_closed: false,
        },
    )
}

/// Shared broker client that feeds requests into an unbounded channel
/// Will be removed once I have a better understanding how this API should look
/// and multi-broker support is implemented
#[derive(Debug, Clone)]
pub struct SharedBrokerClient {
    tx: mpsc::UnboundedSender<(i32, oneshot::Sender<Bytes>, Bytes)>,
    next_correlation_id: Arc<AtomicIsize>,
    client_id: Bytes,
}

impl SharedBrokerClient {
    pub fn connect(
        addr: &SocketAddr,
    ) -> impl Future<Item = SharedBrokerClient, Error = std::io::Error> {
        connect(addr).map(|(client, resp)| {
            let (tx, rx) = mpsc::unbounded();
            // TODO: Is this the right "spawn"?
            // TOOD: Error handling
            tokio::spawn(
                rx.map_err(|()| -> std::io::Error { unreachable!() })
                    .forward(client)
                    .map(|_| ())
                    .map_err(|e| Err(e).expect("franzi client error")),
            );
            tokio::spawn(resp.map_err(|e| Err(e).expect("franzi repsonse error")));
            SharedBrokerClient {
                tx,
                next_correlation_id: Arc::new(AtomicIsize::new(1)),
                client_id: String::from("franzi").into(),
            }
        })
    }

    pub fn send<Req: KafkaRequest>(
        &self,
        req: Req,
    ) -> impl Future<Item = Req::Response, Error = Error> {
        //turn request into Bytes
        let req_header = RequestHeader {
            api_key: req.api_key(),
            api_version: req.api_version(),
            // TODO: Check for overflow
            correlation_id: self.next_correlation_id.fetch_add(1, Relaxed) as i32,
            client_id: Some(KafkaString(self.client_id.clone())),
        };

        let len = req_header.len_to_write() + req.len_to_write();
        let mut buf = BytesMut::with_capacity(4 + len);
        (len as i32).write(&mut buf);
        req_header.write(&mut buf);
        req.write(&mut buf);

        let (tx, rx) = oneshot::channel();
        self.tx
            .unbounded_send((req_header.correlation_id, tx, buf.freeze()))
            .expect("SharedBrokerClient: Closed channel");
        rx.map_err(Error::from).and_then(|buf| {
            let mut buf = std::io::Cursor::new(buf);
            future::result(Req::Response::read(&mut buf).map_err(Error::from))
        })
    }
}

impl<Si, Ei> Sink for BrokerClient<Si, Ei>
where
    Si: Sink<SinkItem = Bytes, SinkError = Ei>,
{
    // TODO: These types don't make sense at all
    type SinkItem = (i32, oneshot::Sender<Bytes>, Bytes); // This should be KafkaRequest?
    type SinkError = Ei; // This should be KafkaError?

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        match self.sink.start_send(item.2) {
            Ok(AsyncSink::Ready) => (),
            Ok(AsyncSink::NotReady(v)) => return Ok(AsyncSink::NotReady((item.0, item.1, v))),
            // pass along errors
            Err(e) => return Err(e),
        };
        // TOOD: Error handling?!
        self.correlations_tx
            .unbounded_send((item.0, item.1))
            .expect("correlations channel closed");
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.sink.poll_complete()
    }
}

impl<St, Et> Future for BrokerResponses<St, Et>
where
    St: futures::Stream<Item = BytesMut, Error = Et>,
{
    type Item = ();
    type Error = Et;

    fn poll(&mut self) -> Poll<(), Et> {
        loop {
            // first, read any new correlation ids that we might need
            while !self.correlations_closed {
                match self.correlations_rx.poll() {
                    Ok(Async::Ready(Some((id, chan)))) => {
                        self.correlations.insert(id, chan);
                    }
                    Ok(Async::Ready(None)) => {
                        // Correlations channel is closed, but continue to read tcp stream as long as there are outstanding requests
                        self.correlations_closed = true;
                    }
                    Ok(Async::NotReady) => break,
                    Err(()) => unreachable!("BrokerResponse got mpsc channel error"),
                };
            }

            let mut buf = match try_ready!(self.stream.poll()) {
                Some(v) => v,
                None => return Ok(Async::Ready(())),
            };

            // read correlation id (TODO: Use ResponseHeader instead?)
            let correlation_id = byteorder::NetworkEndian::read_i32(buf.as_ref());
            buf.advance(4);

            // TODO: Turn into error
            // TODO: Is this a race condition?
            let response_chan = self
                .correlations
                .remove(&correlation_id)
                .expect("BrokerResponse: got unknown correlation id in response");

            // ignore send errors to response_chan. If the recipient is no longer interested, we're neither.
            let _ = response_chan.send(buf.freeze());

            if self.correlations_closed && self.correlations.is_empty() {
                // No remaining outstanding requests
                return Ok(Async::Ready(()));
            }
        }
    }
}

pub struct ConnectionCodec();

impl Encoder for ConnectionCodec {
    // TODO: Is there a better way than this hack?
    type Item = Bytes;
    type Error = std::io::Error;

    fn encode(&mut self, req: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(req.len());
        buf.put(req);
        Ok(())
    }
}

impl Decoder for ConnectionCodec {
    type Item = BytesMut;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.len() < 4 {
            return Ok(None);
        }
        let n = byteorder::NetworkEndian::read_i32(buf.as_ref()) as usize;
        if buf.len() < n + 4 {
            // not long enough (yet)
            return Ok(None);
        }
        buf.advance(4);
        let this = buf.split_off(n);
        let ret = buf.clone();
        *buf = this;
        Ok(Some(ret))
    }
}
