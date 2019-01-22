pub mod types;

use bytes::{BufMut, Bytes};
use std::io::Cursor;

#[derive(Debug)]
pub struct FromBytesError;

#[derive(Debug)]
pub struct ToBytesError;

/// A type that can be constructed from a Kafka Protocol message.
///
/// You usually [`#[derive(FromBytes)]`](../franz_macros/derive.FromBytes.html)
/// or generate the whole type using [`kafka_message!`](../franz_macros/macro.kafka_message.html)
pub trait FromBytes: Sized {
    // Cursor<Bytes> because that allows access to Bytes but also implements Buf
    // Might be fixed in Bytes 0.5,see https://github.com/carllerche/bytes/issues/75
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError>;
}

/// A type that can be serialized to a Kafka Protocol message.
///
/// You usually [`#[derive(ToBytes)]`](../franz_macros/derive.ToBytes.html)
/// or generate the whole type using [`kafka_message!`](../franz_macros/macro.kafka_message.html)
pub trait ToBytes {
    /// How many bytes do I need to reserve so that I can write this message without panicing?
    fn len_to_write(&self) -> usize;

    /// panics if there is not enough capacity in the Buffer
    fn write(&self, bytes: &mut BufMut);
}
