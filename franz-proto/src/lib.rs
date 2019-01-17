#[macro_use]
extern crate franz_macros;

pub mod api_keys;
pub mod messages;
pub mod errors;
pub mod header;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use franz_base::{FromBytes, ToBytes};
    use std::fmt::Debug;
    use std::io::Cursor;

    pub fn write_then_read_eq<T: FromBytes + ToBytes + Eq + Debug>(input: T, expected: &[u8]) {
        let mut buf = BytesMut::with_capacity(input.len_to_write());
        input.write(&mut buf);
        assert_eq!(expected, &buf);
        let parsed: T = FromBytes::read(&mut Cursor::new(buf.freeze())).expect("parse error");
        assert_eq!(input, parsed);
    }
}
