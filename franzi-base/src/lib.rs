#![forbid(unsafe_code)]

pub mod api_keys;
pub mod types;
pub(crate) mod varint;

use bytes::{BufMut, Bytes};
use futures::sync::oneshot::Canceled;
use std::fmt;
use std::io::Cursor;

#[derive(Debug)]
pub struct FromBytesError;

#[derive(Debug)]
pub struct ToBytesError;

/// A type that can be constructed from a Kafka Protocol message.
///
/// You usually [`#[derive(FromKafkaBytes)]`](../franzi_macros/derive.FromKafkaBytes.html)
/// or generate the whole type using [`kafka_message!`](../franzi_macros/macro.kafka_message.html)
pub trait FromKafkaBytes: Sized {
    // Cursor<Bytes> because that allows access to Bytes but also implements Buf
    // Might be fixed in Bytes 0.5,see https://github.com/carllerche/bytes/issues/75
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError>;
}

/// A type that can be serialized to a Kafka Protocol message.
///
/// You usually [`#[derive(ToKafkaBytes)]`](../franzi_macros/derive.ToKafkaBytes.html)
/// or generate the whole type using [`kafka_message!`](../franzi_macros/macro.kafka_message.html)
pub trait ToKafkaBytes {
    /// How many bytes do I need to reserve so that I can write this message without panicing?
    fn len_to_write(&self) -> usize;

    /// panics if there is not enough capacity in the Buffer
    fn write(&self, bytes: &mut BufMut);
}

/// Blanket impl so that you can pass an &ToKafkaBytes to an Framed/Encoder
impl<'a, T: ToKafkaBytes> ToKafkaBytes for &'a T {
    fn len_to_write(&self) -> usize {
        (*self).len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        (*self).write(bytes)
    }
}

/// A Kafka request knows it's own api key and api version, as well its response type
pub trait KafkaRequest: FromKafkaBytes + ToKafkaBytes {
    type Response: FromKafkaBytes + ToKafkaBytes;

    fn api_key(&self) -> i16;

    fn api_version(&self) -> i16;
}

#[derive(Debug)]
pub enum Error {
    FromBytesError,
    ToBytesError,
    Canceled,
    ProtocolError(i16),
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<FromBytesError> for Error {
    fn from(_: FromBytesError) -> Self {
        Error::FromBytesError
    }
}

impl From<ToBytesError> for Error {
    fn from(_: ToBytesError) -> Self {
        Error::ToBytesError
    }
}

impl From<Canceled> for Error {
    fn from(_: Canceled) -> Self {
        Error::Canceled
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ProtocolError(code) => write!(f, "protocol error response {}", code),
            _ => write!(f, "{}", std::error::Error::description(self)),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::FromBytesError => "error reading kafka message",
            Error::ToBytesError => "error writing kafka message",
            Error::Canceled => "response Canceled (connection closed)",
            Error::ProtocolError(_) => "protocol error response",
            Error::Io(_) => "io error",
            Error::Utf8(_) => "utf8 error",
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
