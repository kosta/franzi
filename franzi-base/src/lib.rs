#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

pub mod api_keys;
pub mod types;
pub(crate) mod varint;

use bytes::{BufMut, Bytes};
use futures::channel::{mpsc::SendError, oneshot::Canceled};
use std::fmt;
use std::io;
use std::io::Cursor;
use std::error::Error as _;

pub use varint::{read_vari64, read_varu64, write_vari64, write_varu64};

#[derive(Debug)]
pub enum DecompressionError {
    Gzip(io::Error),
    Snappy(snap::Error),
    Lz4(io::Error),
    Zstd(io::Error),
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecompressionError::*;
        match self {
           Gzip(e) => write!(f, "Franzi: Gzip compression error: {}", e),
           Snappy(e) => write!(f, "Franzi: Snappy compression error: {}", e),
           Lz4(e) => write!(f, "Franzi: Lz4 compression error: {}", e),
           Zstd(e) => write!(f, "Franzi: Zstd compression error: {}", e),
        }
    }
}

impl std::error::Error for DecompressionError {
    fn description(&self) -> &str {
        use DecompressionError::*;
        match self {
           Gzip(_) => "Franzi: Gzip compression error",
           Snappy(_) => "Franzi: Snappy compression error",
           Lz4(_) => "Franzi: Lz4 compression error",
           Zstd(_) => "Franzi: Zstd compression error",
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        // TODO: Put wrapped errors in here?
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // TODO: Put wrapped errors in here?
        None
    }
}

#[derive(Debug)]
pub enum FromBytesError {
    UnexpectedEOF,
    UnexpectedNull,
    UnknownMagicByte(i8),
    UnknownCompression(i8),
    VarIntOverflow,
    Unimplemented(&'static str),
    Decompression(DecompressionError),
}

impl fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FromBytesError::*;
        match self {
            UnexpectedEOF | UnexpectedNull | VarIntOverflow => write!(f, "{}", self.description()),
            UnknownMagicByte(byte)  => write!(f, "Franzi: unknown magic (version) byte {}", byte),
            UnknownCompression(compression)  => write!(f, "Franzi: unknown compression {}", compression),
            Unimplemented(name)  => write!(f, "Franzi: unimplemented: {:?}", name),
            Decompression(e)  => write!(f, "Franzi: decompression error: {}", e),
        }
    }
}

impl std::error::Error for FromBytesError {
    fn description(&self) -> &str {
        use FromBytesError::*;
        match self {
            UnexpectedEOF => "Franzi: unexpected end of message",
            UnexpectedNull => "Franzi: unexpected null length string or array",
            UnknownMagicByte(_) => "Franzi: unknown magic (version) byte",
            UnknownCompression(_) => "Franzi: unkonwn compression",
            VarIntOverflow => "Franzi: var int overflow (too long)",
            Unimplemented(_) => "Franzi: unimplemented",
            Decompression(_) => "Franzi: decompression error",
            }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        // TODO: Put wrapped errors in here?
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // TODO: Put wrapped errors in here?
        None
    }
}

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
    fn write(&self, bytes: &mut dyn BufMut);
}

/// Blanket impl so that you can pass an &ToKafkaBytes to an Framed/Encoder
impl<'a, T: ToKafkaBytes> ToKafkaBytes for &'a T {
    fn len_to_write(&self) -> usize {
        (*self).len_to_write()
    }

    fn write(&self, bytes: &mut dyn BufMut) {
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
    FromBytes(FromBytesError),
    Canceled,
    SendError,
    Protocol(i16),
    Io(io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<FromBytesError> for Error {
    fn from(e: FromBytesError) -> Self {
        Error::FromBytes(e)
    }
}

impl From<Canceled> for Error {
    fn from(_: Canceled) -> Self {
        Error::Canceled
    }
}

impl From<SendError> for Error {
    fn from(_: SendError) -> Self {
        Error::SendError
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
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
        use Error::*;
        match self {
            FromBytes(e) => e.fmt(f),
            Canceled | SendError => write!(f, "{}", self.description()),
            Protocol(code) => write!(f, "Franzi: protocol error response {}", code),
            Io(e) => write!(f, "Franzi: io error: {}", e),
            Utf8(e) => write!(f, "Franzi: utf-8 error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::FromBytes(e) => e.description(),
            Error::Canceled => "Franzi: response Canceled (connection closed)",
            Error::SendError => "Franzi: broker channel closed (connection closed)",
            Error::Protocol(_) => "Franzi: protocol error response",
            Error::Io(e) => e.description(),
            Error::Utf8(e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        // TODO: Put wrapped errors in here?
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // TODO: Put wrapped errors in here?
        None
    }
}
