//! Kafka client
//! Note: This needs better type tetris, I'm really not happy how this is working out.

#![forbid(unsafe_code)]

use byteorder::ByteOrder;
use bytes::{BufMut, Bytes, BytesMut};
use chashmap::CHashMap;
use franzi_base::types::KafkaString;
use franzi_base::{Error as KafkaError, FromKafkaBytes, KafkaRequest, ToKafkaBytes};
use franzi_proto::header::RequestHeader;
use futures::channel::oneshot;
use futures::{
    task::Context, Future, Poll, Sink, SinkExt, Stream, StreamExt,
};
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::codec::{Decoder, Encoder, FramedRead, FramedWrite};
use tokio::net::{
    tcp::TcpStream,
};
use tokio_net::ToSocketAddrs;
use tokio_io::{AsyncRead, AsyncWrite, split::{ReadHalf, WriteHalf}};

pub struct KafkaProtocolRequest {
    payload: Bytes,
    correlation_id: i32,
    response: oneshot::Sender<Bytes>,
}

pub type BrokerConnection<Si, St> = (
    BrokerSink<FramedWrite<WriteHalf<Si>, ConnectionCodec>, io::Error>,
    BrokerResponses<FramedRead<ReadHalf<St>, ConnectionCodec>>,
);

pub type BrokerTcpConnection = BrokerConnection<TcpStream, TcpStream>;

pub struct BrokerSink<Si, Ei>
where
    Si: Sink<Bytes, Error = Ei> + Unpin,
{
    sink: Si,
    correlation_ids: Arc<CHashMap<i32, oneshot::Sender<Bytes>>>,
    next_correlation_id: i32,
}

pub struct BrokerResponses<St>
where
    St: Stream<Item = Result<BytesMut, io::Error>>,
{
    stream: St,
    correlation_ids: Arc<CHashMap<i32, oneshot::Sender<Bytes>>>,
}

// TODO: DNS Resolution!
pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<BrokerTcpConnection, io::Error> {
    Ok(new_broker_connection(tokio::io::split(TcpStream::connect(addr).await?)))
}

// TOOD: Make this testable by NOT using a TcpStream directly...
pub fn new_broker_connection<T: AsyncRead + AsyncWrite>((read, write): (ReadHalf<T>, WriteHalf<T>)) -> BrokerConnection<T, T> {
    let correlation_ids = Arc::new(CHashMap::new());
    let sink = FramedWrite::new(write, ConnectionCodec());
    let stream = FramedRead::new(read, ConnectionCodec());
    (
        BrokerSink {
            sink,
            correlation_ids: correlation_ids.clone(),
            next_correlation_id: 0,
        },
        BrokerResponses {
            stream,
            correlation_ids,
        },
    )
}

impl<Si, Ei> BrokerSink<Si, Ei>
where
    Si: Sink<Bytes, Error = Ei> + Unpin,
    KafkaError: std::convert::From<Ei>,
{
    #[allow(clippy::wrong_self_convention)] // TODO: Move to Request?
    fn to_kafka_protocol_request<Req: KafkaRequest>(
        &mut self,
        req: Req,
        client_id: Bytes,
    ) -> (
        KafkaProtocolRequest,
        impl Future<Output = Result<Req::Response, KafkaError>>,
    ) {
        // Note: if this message is dropped, these is a "gap" in the correlation_ids, but that should be ok
        let correlation_id = self.next_correlation_id;
        // Overflow is ok, we just assume the old messages are already processed and there are no conv
        self.next_correlation_id = self.next_correlation_id.wrapping_add(1);

        //turn request into Bytes
        let req_header = RequestHeader {
            api_key: req.api_key(),
            api_version: req.api_version(),
            correlation_id,
            // TODO: Dont clone() here
            client_id: Some(KafkaString(client_id)),
        };

        let len = req_header.len_to_write() + req.len_to_write();
        let mut buf = BytesMut::with_capacity(4 + len);
        (len as i32).write(&mut buf);
        req_header.write(&mut buf);
        req.write(&mut buf);

        let (tx, rx) = oneshot::channel();
        (
            KafkaProtocolRequest {
                payload: buf.freeze(),
                correlation_id,
                response: tx,
            },
            async {
                let buf = rx.await?;
                let mut buf = io::Cursor::new(buf);
                Req::Response::read(&mut buf).map_err(|e| e.into())
            },
        )
    }

    /// Send a single message through this BrokerConnection
    /// Note: This uses SinkExt::send, so if you're using multiple items, the
    /// caveat of batching together multiple messages and using poll_ready/start_send/poll_flush
    pub async fn send_one<Req: KafkaRequest>(
        &mut self,
        req: Req,
        client_id: Bytes, //TODO: Atomic?
    ) -> Result<Req::Response, KafkaError> {
        let (request, response) = self.to_kafka_protocol_request(req, client_id);
        self.send(request).await?;
        response.await
    }
}

impl<Si, Ei> Sink<KafkaProtocolRequest> for BrokerSink<Si, Ei>
where
    Si: Sink<Bytes, Error = Ei> + Unpin,
{
    type Error = Ei;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.get_mut().sink).poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: KafkaProtocolRequest) -> Result<(), Self::Error> {
        self.correlation_ids
            .insert(item.correlation_id, item.response);
        Pin::new(&mut self.get_mut().sink).start_send(item.payload)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.get_mut().sink).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.get_mut().sink).poll_close(cx)
    }
}

impl<St> BrokerResponses<St>
where
    St: Stream<Item = Result<BytesMut, io::Error>> + Unpin,
{
    pub async fn run(mut self) -> Result<(), io::Error> {
        loop {
            let mut buf = match self.stream.next().await {
                Some(v) => v,
                None => break,
            }?;

            // read correlation id (TODO: Use ResponseHeader instead?)
            let correlation_id = byteorder::NetworkEndian::read_i32(buf.as_ref());
            buf.advance(4);

            // TODO: Turn into error? Log this?
            let response_chan = self
                .correlation_ids
                .remove(&correlation_id)
                .expect("BrokerResponse: got unknown correlation id in response");

            // ignore send errors to response_chan. If the recipient is no longer interested, we're neither.
            let _ = response_chan.send(buf.freeze());
        }
        Ok(())
    }
}

pub struct ConnectionCodec();

impl Encoder for ConnectionCodec {
    // TODO: Is there a better way than this hack?
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, req: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(req.len());
        buf.put(req);
        Ok(())
    }
}

impl Decoder for ConnectionCodec {
    type Item = BytesMut;
    type Error = io::Error;

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
