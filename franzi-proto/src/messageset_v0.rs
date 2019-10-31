use bytes::{BufMut, Bytes};
use franzi_base::{FromBytesError, FromKafkaBytes, ToKafkaBytes};
use std::{convert::TryFrom, io::Cursor};

/// MessageSet (Version: 0) => [offset message_size message]
///    offset => INT64
///    message_size => INT32
///    message => crc magic_byte attributes key value
///        crc => INT32
///        magic_byte => INT8
///        attributes => INT8
///            bit 0~2:
///                0: no compression
///                1: gzip
///                2: snappy
///            bit 3~7: unused
///        key => BYTES
///        value => BYTES
#[derive(Debug, PartialEq, Eq)]

pub struct MessageSetV0 {
    pub offset: i64,
    pub message_size: i32,
    pub messages: Vec<MessageV0>,
}

impl ToKafkaBytes for MessageSetV0 {
    fn len_to_write(&self) -> usize {
        unimplemented!("TODO: MessageSetV0::len_to_write")
    }

    fn write(&self, _bytes: &mut dyn BufMut) {
        unimplemented!("TODO: MessageSetV0::write")
    }
}

impl FromKafkaBytes for MessageSetV0 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let offset: i64 = FromKafkaBytes::read(bytes)?;
        let message_size: i32 = FromKafkaBytes::read(bytes)?;
        let mut messages = Vec::with_capacity(usize::try_from(message_size).unwrap_or_default());
        if message_size > 0 {
            for _ in 0..message_size {
                messages.push(MessageV0::read(bytes)?);
            }
        }
        Ok(MessageSetV0 {
            offset,
            message_size,
            messages,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MessageV0 {
    pub crc: i32,
    pub magic_byte: i8,
    pub attributes: i8,
    pub key: Bytes,
    pub value: Bytes,
}

impl ToKafkaBytes for MessageV0 {
    fn len_to_write(&self) -> usize {
        unimplemented!("TODO: MessageV0::len_to_write")
    }

    fn write(&self, _bytes: &mut dyn BufMut) {
        unimplemented!("TODO: MessageV0::write")
    }
}

impl FromKafkaBytes for MessageV0 {
    fn read(_bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        // let crc: i32 = FromKafkaBytes::read(bytes)?;
        // let magic_byte: i8 = FromKafkaBytes::read(bytes)?;
        // let attributes: i8 = FromKafkaBytes::read(bytes)?;

        // let compression = crate::record::Compression::try_from(attributes as i16)
        //     .map_err(|x| FromBytesError::UnknownCompression(x))?;
        // let decompressed_bytes = compression.decompress(bytes)?;
        // let mut decompressed = Cursor::new(decompressed_bytes);

        // let key = FromKafkaBytes::read(&mut decompressed)?;
        // let value = FromKafkaBytes::read(&mut decompressed)?;

        // Ok(MessageV0 { crc, magic_byte, attributes, key, value })
        unimplemented!("TODO: MessageV0::read")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bytes::Bytes;
//     use franzi_base::FromKafkaBytes;
//     use lazy_static::lazy_static;
//     use std::io::Cursor;
//     use crate::record::Records;

//     struct Test {
//         name: &'static str,
//         expected: Records,
//         encoded: &'static [u8],
//     }

//     lazy_static! {
//             static ref TESTS: [Test; 6] = [
//             Test {
//                 name: "empty message",
//                 expected: Records::V0(MessageSetV0{
//                     offset: 0,
//                     messages: Vec::new(),
//                 }),
//                 encoded: &[
//                     0, 0, 0, 0, 0, 0, 0, 0, // First Offset
//                     0, 0, 0, 49, // Length
//                     0, 0, 0, 0, // Partition Leader Epoch
//                     2, // Version
//                     89, 95, 183, 221, // CRC
//                     0, 0, // Attributes
//                     0, 0, 0, 0, // Last Offset Delta
//                     0, 0, 0, 0, 0, 0, 0, 0, // First Timestamp
//                     0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
//                     0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
//                     0, 0, // Producer Epoch
//                     0, 0, 0, 0, // First Sequence
//                     0, 0, 0, 0, // Number of Records
//                 ],
//             },
//         ];
//     }
// }
