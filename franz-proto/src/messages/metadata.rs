use bytes::{BufMut, Bytes};
use franz_base::types::KafkaString;
use franz_base::{FromBytes, FromBytesError, ToBytes};
use std::io::Cursor;

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct MetadataRequest7 {
    /// An array of topics to fetch metadata for. If the topics array is null fetch metadata for all topics.
    topics: Option<Vec<KafkaString>>,
    /// If this and the broker config auto.create.topics.enable are true, topics that don't exist will be created by the broker. Otherwise, no topics will be created by the broker.
    allow_auto_topic_creation: bool,
}

// Metadata Response (Version: 7) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
//   throttle_time_ms => INT32
//   brokers => node_id host port rack
//     node_id => INT32
//     host => STRING
//     port => INT32
//     rack => NULLABLE_STRING
//   cluster_id => NULLABLE_STRING
//   controller_id => INT32
//   topic_metadata => error_code topic is_internal [partition_metadata]
//     error_code => INT16
//     topic => STRING
//     is_internal => BOOLEAN
//     partition_metadata => error_code partition leader leader_epoch [replicas] [isr] [offline_replicas]
//       error_code => INT16
//       partition => INT32
//       leader => INT32
//       leader_epoch => INT32
//       replicas => INT32
//       isr => INT32
//       offline_replicas => INT32

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct MetadataResponse7 {
//     throttle_time_ms: i32,
//     brokers: MetadataResponseBrokers7,
//     cluster_id: Option<KafkaString>,
//     controller_id: i32,
//     topic_metadata: MetadataResponseTopics7,
// }

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct MetadataResponseBrokers7 {
//     node_id: i32,
//     host: KafkaString,
//     port: i32,
//     rack: Option<KafkaString>,
// }

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct MetadataResponseTopics7 {}
