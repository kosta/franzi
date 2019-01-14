pub mod types;

use std::io::Cursor;
use bytes::{BufMut, Bytes};

pub struct FromBytesError;
pub struct ToBytesError;

pub trait FromBytes : Sized {
    // Cursor<Bytes> because that allows access to Bytes but also implements Buf
    // Might be fixed in Bytes 0.5,see https://github.com/carllerche/bytes/issues/75
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError>;
}

#[allow(clippy::len_without_is_empty)]
pub trait ToBytes {
    fn len(&self) -> usize;

    // panics if there is not enough capacity in the Buffer
    fn write(&self, bytes: &mut BufMut);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
