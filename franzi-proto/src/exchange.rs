use crate::header::RequestHeader;
use bytes::{Bytes, BytesMut};
use franzi_base::{
    types::KafkaString, Error as KafkaError, FromKafkaBytes, KafkaRequest, ToKafkaBytes,
};
use futures::channel::oneshot;
use std::future::Future;
use std::io;

/// an encoded Request with a response channel
pub struct Exchange {
    pub payload: BytesMut,
    pub response: oneshot::Sender<Bytes>,
}

impl Exchange {
    pub fn set_correlation_id(&mut self, correlation_id: i32) {
        self.payload[8..12].copy_from_slice(&correlation_id.to_be_bytes());
    }
}

pub fn make_exchange<Req: KafkaRequest>(
    req: &Req,
    client_id: Bytes,
) -> (
    Exchange,
    impl Future<Output = Result<Req::Response, KafkaError>>,
) {
    //turn request into Bytes
    let req_header = RequestHeader {
        api_key: req.api_key(),
        api_version: req.api_version(),
        //actual correlation id needs to be filled in other method
        correlation_id: -1,
        client_id: Some(KafkaString(client_id)),
    };

    let len = req_header.len_to_write() + req.len_to_write();
    let mut buf = BytesMut::with_capacity(4 + len);
    (len as i32).write(&mut buf);
    req_header.write(&mut buf);
    req.write(&mut buf);

    let (tx, rx) = oneshot::channel();
    (
        Exchange {
            payload: buf,
            response: tx,
        },
        async move {
            let buf = rx.await?;
            let mut buf = io::Cursor::new(buf);
            Req::Response::read(&mut buf).map_err(|e| e.into())
        },
    )
}
