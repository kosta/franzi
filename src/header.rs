
use std::io::Cursor;
use bytes::{Bytes, BufMut};
use crate::{FromBytes, FromBytesError, ToBytes};
use crate::types::KafkaString;

pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<KafkaString>,
}

// TODO: Derive these!
impl FromBytes for RequestHeader {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(RequestHeader {
            api_key: FromBytes::read(bytes)?,
            api_version: FromBytes::read(bytes)?,
            correlation_id: FromBytes::read(bytes)?,
            client_id: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for RequestHeader {
    fn len_to_write(&self) -> usize {
        self.api_key.len_to_write() +
        self.api_version.len_to_write() +
        self.correlation_id.len_to_write() +
        self.client_id.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.api_key.write(bytes);
        self.api_version.write(bytes);
        self.correlation_id.write(bytes);
        self.client_id.write(bytes);
    }
}

pub struct ResponseHeader {
    pub correlation_id: i32,
}

// TODO: Derive these!
impl FromBytes for ResponseHeader {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ResponseHeader {
            correlation_id: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ResponseHeader {
    fn len_to_write(&self) -> usize {
        self.correlation_id.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.correlation_id.write(bytes);
    }
}
