use bytes::{BufMut, Bytes};
use franz_base::types::KafkaString;
use franz_base::{FromBytes, FromBytesError, ToBytes};
use std::io::Cursor;

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<KafkaString>,
}

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ResponseHeader {
    pub correlation_id: i32,
}
