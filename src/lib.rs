pub mod types;

use bytes::{Buf, BufMut};

pub struct FromBufError;
pub struct ToBytesError;

pub trait FromBuf : Sized {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError>;
}

// TODO: Is this IntoBuf?
pub trait ToBuf {
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
