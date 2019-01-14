pub mod api_keys;
pub mod api_versions;
pub mod errors;
pub mod header;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
