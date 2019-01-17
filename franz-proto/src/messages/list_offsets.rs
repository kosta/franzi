use franz_base::{FromBytes, FromBytesError, ToBytes};
use bytes::{BufMut, Bytes};
use std::io::Cursor;

use franz_base::types::KafkaString;

// ListOffsets Request (Version: 4) => replica_id isolation_level [topics]
//   replica_id => INT32
//   isolation_level => INT8
//   topics => topic [partitions]
//     topic => STRING
//     partitions => partition current_leader_epoch timestamp
//       partition => INT32
//       current_leader_epoch => INT32
//       timestamp => INT64

#[derive(Debug)]
pub struct ListOffsetsRequest4 {
    /// Broker id of the follower. For normal consumers, use -1.
    pub replica_id: i32,
    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    pub isolation_level: i8,
    /// Topics to list offsets.
    pub topics: Option<Vec<ListOffsetsRequest4Topic>>,
}

impl FromBytes for ListOffsetsRequest4 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsRequest4 {
            replica_id: FromBytes::read(bytes)?,
            isolation_level: FromBytes::read(bytes)?,
            topics: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsRequest4 {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.replica_id) +
        ToBytes::len_to_write(&self.isolation_level) +
        ToBytes::len_to_write(&self.topics)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.replica_id, bytes);
        ToBytes::write(&self.isolation_level, bytes);
        ToBytes::write(&self.topics, bytes);
    }
}

#[derive(Debug)]
pub struct ListOffsetsRequest4Topic {
    /// Name of topic
    pub topic: KafkaString,
    /// Partitions to list offsets.
    pub partitions: Option<Vec<ListOffsetsRequest4Partition>>,
}

impl FromBytes for ListOffsetsRequest4Topic {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsRequest4Topic {
            topic: FromBytes::read(bytes)?,
            partitions: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsRequest4Topic {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.topic) +
        ToBytes::len_to_write(&self.partitions)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.topic, bytes);
        ToBytes::write(&self.partitions, bytes);
    }
}

#[derive(Debug)]
pub struct ListOffsetsRequest4Partition {
      /// Topic partition id
      pub partition: i32,
      ///The current leader epoch, if provided, is used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
      pub current_leader_epoch: i32,
      /// The target timestamp for the partition.
      pub timestamp: i64,
}

impl FromBytes for ListOffsetsRequest4Partition {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsRequest4Partition {
            partition: FromBytes::read(bytes)?,
            current_leader_epoch: FromBytes::read(bytes)?,
            timestamp: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsRequest4Partition {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.partition) +
        ToBytes::len_to_write(&self.current_leader_epoch) +
        ToBytes::len_to_write(&self.timestamp)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.partition, bytes);
        ToBytes::write(&self.current_leader_epoch, bytes);
        ToBytes::write(&self.timestamp, bytes);
    }
}

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

#[derive(Debug)]
pub struct ListOffsetsResponse4 {
    /// Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
    pub throttle_time_ms: i32,
    /// The listed offsets by topic
    /// TODO: Is this better names as `topics`?
    pub responses: Option<Vec<ListOffsetsResponse4Responses>>,
}

impl FromBytes for ListOffsetsResponse4 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsResponse4 {
            throttle_time_ms: FromBytes::read(bytes)?,
            responses: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsResponse4 {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.throttle_time_ms) +
        ToBytes::len_to_write(&self.responses)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.throttle_time_ms, bytes);
        ToBytes::write(&self.responses, bytes);
    }
}

#[derive(Debug)]
pub struct ListOffsetsResponse4Responses {
    /// Name of topic
    pub topic: KafkaString,
    // The listed offsets by partition
    pub partition_responses: Option<Vec<ListOffsetsResponse4PartitionResponses>>,
}

impl FromBytes for ListOffsetsResponse4Responses {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsResponse4Responses {
            topic: FromBytes::read(bytes)?,
            partition_responses: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsResponse4Responses {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.topic) +
        ToBytes::len_to_write(&self.partition_responses)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.topic, bytes);
        ToBytes::write(&self.partition_responses, bytes);
    }
}

#[derive(Debug)]
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

impl FromBytes for ListOffsetsResponse4PartitionResponses {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ListOffsetsResponse4PartitionResponses {
            partition: FromBytes::read(bytes)?,
            error_code: FromBytes::read(bytes)?,
            timestamp: FromBytes::read(bytes)?,
            offset: FromBytes::read(bytes)?,
            leader_epoch: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ListOffsetsResponse4PartitionResponses {
    fn len_to_write(&self) -> usize {
        ToBytes::len_to_write(&self.partition) +
        ToBytes::len_to_write(&self.error_code) +
        ToBytes::len_to_write(&self.timestamp) +
        ToBytes::len_to_write(&self.offset) +
        ToBytes::len_to_write(&self.leader_epoch)
    }


    fn write(&self, bytes: &mut BufMut) {
        ToBytes::write(&self.partition, bytes);
        ToBytes::write(&self.error_code, bytes);
        ToBytes::write(&self.timestamp, bytes);
        ToBytes::write(&self.offset, bytes);
        ToBytes::write(&self.leader_epoch, bytes);
    }
}