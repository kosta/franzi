use byteorder::ByteOrder;
use bytes::{Bytes, BufMut, BytesMut};
use futures::Future;
use futures::sync::oneshot;
use tokio::codec::{Decoder, Encoder, Framed};
use tokio::net::tcp::TcpStream;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::prelude::stream::{Stream, SplitSink, SplitStream};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// TODO: Find something more efficient?
pub type Correlations = std::sync::Arc<std::sync::Mutex<HashMap<i32, oneshot::Sender<Bytes>>>>;

pub type BrokerConnection<T> = (BrokerClient<T>, BrokerResponses<T>);

pub struct BrokerClient<T>
    where T: AsyncRead + AsyncWrite,
{
    sink: SplitSink<Framed<T, ConnectionCodec>>,
    correlations: Correlations,
}

pub struct BrokerResponses<T>
    where T: AsyncRead + AsyncWrite,
{
    stream: SplitStream<Framed<T, ConnectionCodec>>,
    correlations: Correlations,
}

pub fn connect(addr: &SocketAddr) -> impl Future<Item=BrokerConnection<TcpStream>, Error=std::io::Error>{
    TcpStream::connect(addr).map(new_broker_connection)
}

pub fn new_broker_connection<T>(sock: T) -> BrokerConnection<T>
    where T: AsyncRead + AsyncWrite,
{
    let correlations = Arc::new(Mutex::new(HashMap::new()));
    let (sink, stream) = Framed::new(sock, ConnectionCodec()).split();
    (BrokerClient {
        sink,
        correlations: correlations.clone(),
    }, BrokerResponses {
        stream,
        correlations,
    })
}

pub struct ConnectionCodec();

// not sure if this makes too much sense?
impl Encoder for ConnectionCodec {
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

