use franzi_base::types::KafkaString;

#[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]
pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<KafkaString>,
}

#[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]
pub struct ResponseHeader {
    pub correlation_id: i32,
}
