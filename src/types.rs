//! Kafka primitive types

use std::mem::size_of;

use bytes::{Buf, BufMut};

use crate::{FromBuf, FromBufError, ToBuf};

/// BOOLEAN	Represents a boolean value in a byte. Values 0 and 1 are used to represent false and true respectively. When reading a boolean value, any non-zero value is considered true.
impl FromBuf for bool {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_u8() != 0)
    }
}

impl ToBuf for bool {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_u8(if *self {1} else {0});
    }
}

/// INT8	Represents an integer between -2^7 and 2^7-1 inclusive.

impl FromBuf for i8 {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_i8())
    }
}

impl ToBuf for i8 {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i8(*self);
    }
}

/// INT16	Represents an integer between -2^15 and 2^15-1 inclusive. The values are encoded using two bytes in network byte order (big-endian).

impl FromBuf for i16 {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_i16_be())
    }
}

impl ToBuf for i16 {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i16_be(*self)
    }
}

/// INT32	Represents an integer between -2^31 and 2^31-1 inclusive. The values are encoded using four bytes in network byte order (big-endian).

impl FromBuf for i32 {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_i32_be())
    }
}

impl ToBuf for i32 {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i32_be(*self)
    }
}

/// INT64	Represents an integer between -2^63 and 2^63-1 inclusive. The values are encoded using eight bytes in network byte order (big-endian).
impl FromBuf for i64 {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_i64_be())
    }
}

impl ToBuf for i64 {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i64_be(*self)
    }
}

/// UINT32	Represents an integer between 0 and 2^32-1 inclusive. The values are encoded using four bytes in network byte order (big-endian).

impl FromBuf for u32 {
    fn read(bytes: &mut Buf) -> Result<Self, FromBufError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBufError);
        }
        Ok(bytes.get_u32_be())
    }
}

impl ToBuf for u32 {
    fn len(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_u32_be(*self)
    }
}

/// VARINT	Represents an integer between -2^31 and 2^31-1 inclusive. Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
/// TODO: Implement VARINT

/// VARLONG	Represents an integer between -2^63 and 2^63-1 inclusive. Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
/// TODO: Implement VARLONG

/// STRING	Represents a sequence of characters. First the length N is given as an INT16. Then N bytes follow which are the UTF-8 encoding of the character sequence. Length must not be negative.


/// NULLABLE_STRING	Represents a sequence of characters or null. For non-null strings, first the length N is given as an INT16. Then N bytes follow which are the UTF-8 encoding of the character sequence. A null value is encoded with length of -1 and there are no following bytes.

/// BYTES	Represents a raw sequence of bytes. First the length N is given as an INT32. Then N bytes follow.

/// NULLABLE_BYTES	Represents a raw sequence of bytes or null. For non-null values, first the length N is given as an INT32. Then N bytes follow. A null value is encoded with length of -1 and there are no following bytes.

/// RECORDS	Represents a sequence of Kafka records as NULLABLE_BYTES. For a detailed description of records see Message Sets.

/// ARRAY	Represents a sequence of objects of a given type T. Type T can be either a primitive type (e.g. STRING) or a structure. First, the length N is given as an INT32. Then N instances of type T follow. A null array is represented with a length of -1. In protocol documentation an array of T instances is referred to as [T].
pub struct ARRAY { //<T: FromBuf + ToBuf> {
    //TODO: Implement me!
}