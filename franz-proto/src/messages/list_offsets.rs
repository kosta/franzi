use franz_base::types::KafkaString;

kafka_message!(
    "ListOffsets Request (Version: 4) => replica_id isolation_level [topics]
  replica_id => INT32
  isolation_level => INT8
  topics => topic [partitions]
    topic => STRING
    partitions => partition current_leader_epoch timestamp
      partition => INT32
      current_leader_epoch => INT32
      timestamp => INT64

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
isolation_level	This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
current_leader_epoch	The current leader epoch, if provided, is used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
timestamp	The target timestamp for the partition.");

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct ListOffsetsRequest4 {
//     /// Broker id of the follower. For normal consumers, use -1.
//     pub replica_id: i32,
//     /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
//     pub isolation_level: i8,
//     /// Topics to list offsets.
//     pub topics: Option<Vec<ListOffsetsRequest4Topic>>,
// }

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct ListOffsetsRequest4Topic {
//     /// Name of topic
//     pub topic: KafkaString,
//     /// Partitions to list offsets.
//     pub partitions: Option<Vec<ListOffsetsRequest4Partition>>,
// }

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct ListOffsetsRequest4Partition {
//     /// Topic partition id
//     pub partition: i32,
//     ///The current leader epoch, if provided, is used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
//     pub current_leader_epoch: i32,
//     /// The target timestamp for the partition.
//     pub timestamp: i64,
// }

// ListOffsets Response (Version: 4) => throttle_time_ms [responses]
//   throttle_time_ms => INT32
//   responses => topic [partition_responses]
//     topic => STRING
//     partition_responses => partition error_code timestamp offset leader_epoch
//       partition => INT32
//       error_code => INT16
//       timestamp => INT64
//       offset => INT64
//       leader_epoch => INT32

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ListOffsetsResponse4 {
    /// Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
    pub throttle_time_ms: i32,
    /// The listed offsets by topic
    /// TODO: Is this better names as `topics`?
    pub responses: Option<Vec<ListOffsetsResponse4Responses>>,
}

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ListOffsetsResponse4Responses {
    /// Name of topic
    pub topic: KafkaString,
    // The listed offsets by partition
    pub partition_responses: Option<Vec<ListOffsetsResponse4PartitionResponses>>,
}

#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ListOffsetsResponse4PartitionResponses {
    /// Topic partition id
    pub partition: i32,
    ///Response error code
    pub error_code: i16,
    ///The timestamp associated with the returned offset
    pub timestamp: i64,
    ///The offset found
    pub offset: i64,
    ///The leader epoch
    pub leader_epoch: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::write_then_read_eq;

    #[test]
    #[allow(non_snake_case)]
    fn test_ListOffsetsRequest4() {
        write_then_read_eq(ListOffsetsRequest4{
            replica_id: 7,
            isolation_level: 1,
            topics: Some(vec![
                ListOffsetsRequest4Topic{
                    topic: KafkaString(String::from("some-topic").into()),
                    partitions: Some(vec![
                        ListOffsetsRequest4Partition{
                            partition: 0,
                            current_leader_epoch: 1,
                            timestamp: 2,
                        },
                        ListOffsetsRequest4Partition{
                            partition: 823664,
                            current_leader_epoch: 237263,
                            timestamp: 20938123,
                        }
                    ]),
                },
            ])
        },
        b"\0\0\0\x07\x01\0\0\0\x01\0\nsome-topic\0\0\0\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02\0\x0c\x91p\0\x03\x9e\xcf\0\0\0\0\x01?}\x8b");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ListOffsetsResponse4() {
        write_then_read_eq(ListOffsetsResponse4{
            throttle_time_ms: 0,
            responses: Some(vec![
                ListOffsetsResponse4Responses{
                    topic: KafkaString(String::from("some-topic").into()),
                    partition_responses: Some(vec![
                        ListOffsetsResponse4PartitionResponses{
                            partition: 0,
                            error_code: 0,
                            timestamp: 8489283,
                            offset: 2131,
                            leader_epoch: 12321,
                        }
                    ]),
                },
            ])
        },
        b"\0\0\0\0\0\0\0\x01\0\nsome-topic\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x81\x89C\0\0\0\0\0\0\x08S\0\00!");
    }
}
