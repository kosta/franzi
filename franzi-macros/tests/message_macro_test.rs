// use bytes::Bytes;
// use franzi_macros::kafka_message;
// use std::io::Cursor;

// kafka_message!("Foo");

// #[test]
// fn test_message() {
//     let f: Foo = franzi_base::FromKafkaBytes::read(&mut Cursor::new(Bytes::from(&b""[..]))).unwrap();
//     assert_eq!(franzi_base::ToKafkaBytes::len_to_write(&f), 0);
//     assert_eq!(format!("foo: {:?}", f), "foo: Foo");
//     assert_eq!(f, Foo{});
// }
