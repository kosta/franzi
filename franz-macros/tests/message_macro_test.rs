use franz_macros::message;
use std::io::Cursor;
use bytes::Bytes;

message!("Foo");

#[test]
fn test_message() {
    let f: Foo = franz_base::FromBytes::read(&mut Cursor::new(Bytes::from(&b""[..]))).unwrap();
    assert_eq!(franz_base::ToBytes::len_to_write(&f), 0);
    assert_eq!(format!("foo: {:?}", f), "foo: Foo");
    assert_eq!(f, Foo{});
}
