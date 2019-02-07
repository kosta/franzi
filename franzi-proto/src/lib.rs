#![forbid(unsafe_code)]

#[macro_use]
extern crate franzi_macros;

pub mod errors;
pub mod header;
pub mod messages;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use franzi_base::{FromKafkaBytes, ToKafkaBytes};
    use std::fmt::Debug;
    use std::io::Cursor;

    pub fn write_then_read_eq<T: FromKafkaBytes + ToKafkaBytes + Eq + Debug>(
        input: T,
        expected: &[u8],
    ) {
        let mut buf = BytesMut::with_capacity(input.len_to_write());
        input.write(&mut buf);
        assert_eq!(expected, &buf);
        let parsed: T = FromKafkaBytes::read(&mut Cursor::new(buf.freeze())).expect("parse error");
        assert_eq!(input, parsed);
    }
}
