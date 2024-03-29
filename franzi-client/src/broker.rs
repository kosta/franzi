use bytes::{Buf, Bytes, BytesMut};
use chashmap::CHashMap;
use franzi_base::{Error as KafkaError, KafkaRequest};
use franzi_proto::exchange;
use futures::channel::oneshot;
use futures::{task::Context, Sink, SinkExt, Stream, StreamExt};
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::Poll;
use tokio::io::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tokio_util::codec::{FramedRead, FramedWrite};

pub type BrokerConnection<Si, St> = (
    BrokerSink<FramedWrite<WriteHalf<Si>, LengthDelimitedCodec>>,
    BrokerResponses<FramedRead<ReadHalf<St>, LengthDelimitedCodec>>,
);

pub type BrokerTcpConnection = BrokerConnection<TcpStream, TcpStream>;
pub type BrokerTcpSink = BrokerSink<FramedWrite<WriteHalf<TcpStream>, LengthDelimitedCodec>>;

pub struct BrokerSink<Si>
where
    Si: Sink<Bytes, Error = io::Error> + Unpin,
{
    sink: Si,
    /// weak (instead of arc) so we notice if the BrokerResponses are closed
    correlation_ids: Weak<CHashMap<i32, oneshot::Sender<Bytes>>>,
    // todo: better naming?
    _next_correlation_id: i32,
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
    Ok(new_broker_connection(tokio::io::split(
        TcpStream::connect(addr).await?,
    )))
}

// TOOD: Make this testable by NOT using a TcpStream directly...
pub fn new_broker_connection<T: AsyncRead + AsyncWrite>(
    (read, write): (ReadHalf<T>, WriteHalf<T>),
) -> BrokerConnection<T, T> {
    let correlation_ids = Arc::new(CHashMap::new());
    let sink = FramedWrite::new(write, LengthDelimitedCodec::new());
    let stream = FramedRead::new(read, LengthDelimitedCodec::new());
    (
        BrokerSink {
            sink,
            correlation_ids: Arc::downgrade(&correlation_ids),
            _next_correlation_id: 0,
        },
        BrokerResponses {
            stream,
            correlation_ids,
        },
    )
}

impl<Si> BrokerSink<Si>
where
    Si: Sink<Bytes, Error = io::Error> + Unpin,
{
    fn next_correlation_id(&mut self) -> i32 {
        // Note: if this message is dropped, these is a "gap" in the correlation_ids, but that should be ok
        let correlation_id = self._next_correlation_id;
        // Overflow is ok, we just assume the old messages are already processed and there are no conv
        self._next_correlation_id = self._next_correlation_id.wrapping_add(1);
        correlation_id
    }

    /// Send a single message through this BrokerConnection
    /// Note: This uses SinkExt::send, so if you're using multiple items, the
    /// caveat of batching together multiple messages and using poll_ready/start_send/poll_flush
    pub async fn send_one<Req: KafkaRequest>(
        &mut self,
        req: Req,
        client_id: Bytes, //TODO: Atomic?
    ) -> Result<Req::Response, KafkaError> {
        let (request, response) = exchange::make_exchange(&req, client_id);
        self.send(request).await?;
        response.await
    }
}

impl<Si> Sink<exchange::Exchange> for BrokerSink<Si>
where
    Si: Sink<Bytes, Error = io::Error> + Unpin,
{
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.get_mut().sink).poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, mut item: exchange::Exchange) -> Result<(), Self::Error> {
        let this = self.get_mut();
        // TODO: Is this too expensive?
        let correlation_ids = this.correlation_ids.upgrade().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionReset,
                "broker responses are closed",
            )
        })?;
        let correlation_id = this.next_correlation_id();
        item.set_correlation_id(correlation_id);
        correlation_ids.insert(correlation_id, item.response);
        Pin::new(&mut this.sink).start_send(item.payload.freeze())
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
            let correlation_id = buf.get_i32();

            // TODO: Turn into error? Log this?
            let response_chan = self
                .correlation_ids
                .remove(&correlation_id)
                .expect("BrokerResponse: got unknown correlation id in response");

            // ignore send errors to response_chan. If the recipient is no longer interested, we're neither.
            let _ = response_chan.send(buf.freeze());

            if self.correlation_ids.len() == 0 && Arc::weak_count(&self.correlation_ids) == 0 {
                // "sending half" is closed, nothing more to do...
                // TODO: Is this a too slow and too hacky way to determine this?
                break;
            }
        }

        Ok(())
    }
}
