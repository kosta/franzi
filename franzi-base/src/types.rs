//! Kafka primitive types

use std::fmt;
use std::io::Cursor;
use std::mem::size_of;

use bytes::{Buf, BufMut, Bytes};

use crate::{FromKafkaBytes, FromBytesError, ToKafkaBytes};

/// BOOLEAN	Represents a boolean value in a byte. Values 0 and 1 are used to represent false and true respectively. When reading a boolean value, any non-zero value is considered true.
impl FromKafkaBytes for bool {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_u8() != 0)
    }
}

impl ToKafkaBytes for bool {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_u8(if *self { 1 } else { 0 });
    }
}

/// INT8	Represents an integer between -2^7 and 2^7-1 inclusive.

impl FromKafkaBytes for i8 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_i8())
    }
}

impl ToKafkaBytes for i8 {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i8(*self);
    }
}

/// INT16	Represents an integer between -2^15 and 2^15-1 inclusive. The values are encoded using two bytes in network byte order (big-endian).

impl FromKafkaBytes for i16 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_i16_be())
    }
}

impl ToKafkaBytes for i16 {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i16_be(*self)
    }
}

/// INT32	Represents an integer between -2^31 and 2^31-1 inclusive. The values are encoded using four bytes in network byte order (big-endian).

impl FromKafkaBytes for i32 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_i32_be())
    }
}

impl ToKafkaBytes for i32 {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i32_be(*self)
    }
}

/// INT64	Represents an integer between -2^63 and 2^63-1 inclusive. The values are encoded using eight bytes in network byte order (big-endian).
impl FromKafkaBytes for i64 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_i64_be())
    }
}

impl ToKafkaBytes for i64 {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_i64_be(*self)
    }
}

/// UINT32	Represents an integer between 0 and 2^32-1 inclusive. The values are encoded using four bytes in network byte order (big-endian).

impl FromKafkaBytes for u32 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        if bytes.remaining() < size_of::<Self>() {
            return Err(FromBytesError);
        }
        Ok(bytes.get_u32_be())
    }
}

impl ToKafkaBytes for u32 {
    fn len_to_write(&self) -> usize {
        size_of::<Self>()
    }

    fn write(&self, bytes: &mut BufMut) {
        bytes.put_u32_be(*self)
    }
}

/// VARINT	Represents an integer between -2^31 and 2^31-1 inclusive. Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
/// TODO: Implement VARINT

#[allow(non_camel_case_types)]
pub struct vi32(i32);

/// VARLONG	Represents an integer between -2^63 and 2^63-1 inclusive. Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
/// TODO: Implement VARLONG

#[allow(non_camel_case_types)]
pub struct vi64(i64);

/// STRING	Represents a sequence of characters. First the length N is given as an INT16. Then N bytes follow which are the UTF-8 encoding of the character sequence. Length must not be negative.
#[derive(Eq, PartialEq)]
pub struct KafkaString(pub Bytes);

impl fmt::Debug for KafkaString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::str::from_utf8(self.0.as_ref()) {
            Ok(s) => write!(f, "KafkaString {{ utf8 {:?} }}", s),
            Err(_) => write!(f, "KafkaString {{ bad {:?} }}", self.0.as_ref()),
        }
    }
}

impl FromKafkaBytes for KafkaString {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        <Option<KafkaString> as FromKafkaBytes>::read(bytes).and_then(|s| s.ok_or(FromBytesError))
    }
}

impl ToKafkaBytes for KafkaString {
    fn len_to_write(&self) -> usize {
        size_of::<i16>() + self.0.len()
    }

    fn write(&self, bytes: &mut BufMut) {
        // TODO: Overflow check?
        bytes.put_i16_be(self.0.len() as i16);
        bytes.put_slice(self.0.as_ref());
    }
}

/// NULLABLE_STRING	Represents a sequence of characters or null. For non-null strings, first the length N is given as an INT16. Then N bytes follow which are the UTF-8 encoding of the character sequence. A null value is encoded with length of -1 and there are no following bytes.

impl FromKafkaBytes for Option<KafkaString> {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let len = i16::read(bytes)?;
        if len < 0 {
            return Ok(None);
        }
        let len = len as usize;
        if bytes.remaining() < len {
            return Err(FromBytesError);
        }
        let pos = bytes.position() as usize;
        let s = KafkaString(bytes.get_ref().slice(pos, pos + len));
        bytes.advance(len);
        Ok(Some(s))
    }
}

impl ToKafkaBytes for Option<KafkaString> {
    fn len_to_write(&self) -> usize {
        match self {
            None => 1,
            Some(s) => s.len_to_write(),
        }
    }

    fn write(&self, bytes: &mut BufMut) {
        match self {
            None => bytes.put_i16_be(-1),
            Some(s) => s.write(bytes),
        }
    }
}

/// BYTES	Represents a raw sequence of bytes. First the length N is given as an INT32. Then N bytes follow.

impl FromKafkaBytes for Bytes {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        <Option<Bytes> as FromKafkaBytes>::read(bytes).and_then(|s| s.ok_or(FromBytesError))
    }
}

impl ToKafkaBytes for Bytes {
    fn len_to_write(&self) -> usize {
        size_of::<i32>() + self.len()
    }

    fn write(&self, bytes: &mut BufMut) {
        // TODO: Overflow check?
        bytes.put_i32_be(self.len() as i32);
        bytes.put_slice(self.as_ref());
    }
}

/// NULLABLE_BYTES	Represents a raw sequence of bytes or null. For non-null values, first the length N is given as an INT32. Then N bytes follow. A null value is encoded with length of -1 and there are no following bytes.

impl FromKafkaBytes for Option<Bytes> {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let len = i32::read(bytes)?;
        if len < 0 {
            return Ok(None);
        }
        let len = len as usize;
        if bytes.remaining() < len {
            return Err(FromBytesError);
        }
        let pos = bytes.position() as usize;
        let s = bytes.get_ref().slice(pos, pos + len);
        bytes.advance(len);
        Ok(Some(s))
    }
}

impl ToKafkaBytes for Option<Bytes> {
    fn len_to_write(&self) -> usize {
        match self {
            None => 1,
            Some(s) => s.len_to_write(),
        }
    }

    fn write(&self, bytes: &mut BufMut) {
        match self {
            None => bytes.put_i32_be(-1),
            Some(s) => s.write(bytes),
        }
    }
}

/// RECORDS	Represents a sequence of Kafka records as NULLABLE_BYTES. For a detailed description of records see Message Sets.

// TODO
pub struct Records {}

/// ARRAY	Represents a sequence of objects of a given type T. Type T can be either a primitive type (e.g. STRING) or a structure. First, the length N is given as an INT32. Then N instances of type T follow. A null array is represented with a length of -1. In protocol documentation an array of T instances is referred to as [T].

// TODO: Revisit whether it's ok (performance wise) to allocate a Vec here? A bit of an issue is the fact that we dont't know the length of this in advance without parsing the data

impl<T: FromKafkaBytes> FromKafkaBytes for Option<Vec<T>> {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let item_len = i32::read(bytes)?;
        if item_len < 0 {
            return Ok(None);
        }
        let item_len = item_len as usize;
        let mut vec = Vec::with_capacity(item_len);
        for _ in 0..item_len {
            vec.push(T::read(bytes)?);
        }
        Ok(Some(vec))
    }
}

impl<T: ToKafkaBytes> ToKafkaBytes for Option<Vec<T>> {
    fn len_to_write(&self) -> usize {
        size_of::<i32>()
            + match self {
                None => 0,
                Some(vec) => vec.iter().map(ToKafkaBytes::len_to_write).sum::<usize>(),
            }
    }

    fn write(&self, bytes: &mut BufMut) {
        // TODO: Overflow check?
        match self {
            None => bytes.put_i32_be(-1),
            Some(vec) => {
                bytes.put_i32_be(vec.len() as i32);
                vec.iter().for_each(|i| i.write(bytes));
            }
        }
    }
}