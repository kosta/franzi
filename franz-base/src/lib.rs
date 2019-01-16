pub mod types;

use bytes::{BufMut, Bytes};
use std::io::Cursor;

#[derive(Debug)]
pub struct FromBytesError;

#[derive(Debug)]
pub struct ToBytesError;

pub trait FromBytes: Sized {
    // Cursor<Bytes> because that allows access to Bytes but also implements Buf
    // Might be fixed in Bytes 0.5,see https://github.com/carllerche/bytes/issues/75
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError>;
}

pub trait ToBytes {
    fn len_to_write(&self) -> usize;

    // panics if there is not enough capacity in the Buffer
    fn write(&self, bytes: &mut BufMut);
}
