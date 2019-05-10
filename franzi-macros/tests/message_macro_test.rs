// use bytes::Bytes;
use franzi_macros::implement_kafka_messages_from_file;
// use std::io::Cursor;

// kafka_message!("Foo");

#[test]
fn test_protocol_html() {
    let ts = implement_kafka_messages_from_file("protocol.html");
}
