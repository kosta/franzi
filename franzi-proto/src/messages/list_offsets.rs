kafka_message!(
    "ListOffsets Request (Version: 0) => replica_id [topics]
  replica_id => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition timestamp max_num_offsets
      partition => INT32
      timestamp => INT64
      max_num_offsets => INT32

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
timestamp	The target timestamp for the partition.
max_num_offsets	Maximum offsets to return."
);

kafka_message!(
    "ListOffsets Request (Version: 1) => replica_id [topics]
  replica_id => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition timestamp
      partition => INT32
      timestamp => INT64

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
timestamp	The target timestamp for the partition."
);

kafka_message!(
    "ListOffsets Request (Version: 2) => replica_id isolation_level [topics]
  replica_id => INT32
  isolation_level => INT8
  topics => topic [partitions]
    topic => STRING
    partitions => partition timestamp
      partition => INT32
      timestamp => INT64

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
isolation_level	This setting controls the visibility of transactional
records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records
visible. With READ_COMMITTED (isolation_level = 1), non-transactional
and COMMITTED transactional records are visible. To be more concrete,
READ_COMMITTED returns all data from offsets smaller than the current
LSO (last stable offset), and enables the inclusion of the list of
aborted transactions in the result, which allows consumers to discard
ABORTED transactional records
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
timestamp	The target timestamp for the partition."
);

kafka_message!(
    "ListOffsets Request (Version: 3) => replica_id isolation_level [topics]
  replica_id => INT32
  isolation_level => INT8
  topics => topic [partitions]
    topic => STRING
    partitions => partition timestamp
      partition => INT32
      timestamp => INT64

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
isolation_level	This setting controls the visibility of transactional
records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records
visible. With READ_COMMITTED (isolation_level = 1), non-transactional
and COMMITTED transactional records are visible. To be more concrete,
READ_COMMITTED returns all data from offsets smaller than the current
LSO (last stable offset), and enables the inclusion of the list of
aborted transactions in the result, which allows consumers to discard
ABORTED transactional records
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
timestamp	The target timestamp for the partition."
);

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
isolation_level	This setting controls the visibility of transactional
records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records
visible. With READ_COMMITTED (isolation_level = 1), non-transactional
and COMMITTED transactional records are visible. To be more concrete,
READ_COMMITTED returns all data from offsets smaller than the current
LSO (last stable offset), and enables the inclusion of the list of
aborted transactions in the result, which allows consumers to discard
ABORTED transactional records
topics	Topics to list offsets.
topic	Name of topic
partitions	Partitions to list offsets.
partition	Topic partition id
current_leader_epoch	The current leader epoch, if provided, is used to
fence consumers/replicas with old metadata. If the epoch provided by the
client is larger than the current epoch known to the broker, then the
UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch
is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
timestamp	The target timestamp for the partition."
);

kafka_message!(
    "ListOffsets Response (Version: 0) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code [offsets]
      partition => INT32
      error_code => INT16
      offsets => INT64

Field 	Description
responses	The listed offsets by topic
topic	Name of topic
partition_responses	The listed offsets by partition
partition	Topic partition id
error_code	Response error code
offsets	A list of offsets."
);

kafka_message!(
    "ListOffsets Response (Version: 1) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code timestamp offset
      partition => INT32
      error_code => INT16
      timestamp => INT64
      offset => INT64

Field 	Description
responses	The listed offsets by topic
topic	Name of topic
partition_responses	The listed offsets by partition
partition	Topic partition id
error_code	Response error code
timestamp	The timestamp associated with the returned offset
offset	The offset found"
);

kafka_message!(
    "ListOffsets Response (Version: 2) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code timestamp offset
      partition => INT32
      error_code => INT16
      timestamp => INT64
      offset => INT64

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	The listed offsets by topic
topic	Name of topic
partition_responses	The listed offsets by partition
partition	Topic partition id
error_code	Response error code
timestamp	The timestamp associated with the returned offset
offset	The offset found"
);

kafka_message!(
    "ListOffsets Response (Version: 3) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code timestamp offset
      partition => INT32
      error_code => INT16
      timestamp => INT64
      offset => INT64

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	The listed offsets by topic
topic	Name of topic
partition_responses	The listed offsets by partition
partition	Topic partition id
error_code	Response error code
timestamp	The timestamp associated with the returned offset
offset	The offset found"
);

kafka_message!(
    "ListOffsets Response (Version: 4) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code timestamp offset leader_epoch
      partition => INT32
      error_code => INT16
      timestamp => INT64
      offset => INT64
      leader_epoch => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	The listed offsets by topic
topic	Name of topic
partition_responses	The listed offsets by partition
partition	Topic partition id
error_code	Response error code
timestamp	The timestamp associated with the returned offset
offset	The offset found
leader_epoch	The leader epoch"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::write_then_read_eq;
    use ::franzi_base::types::KafkaString;

    #[test]
    #[allow(non_snake_case)]
    fn test_ListOffsetsRequestV4() {
        write_then_read_eq(ListOffsetsRequestV4{
            replica_id: 7,
            isolation_level: 1,
            topics: Some(vec![
                ListOffsetsRequestV4_topics{
                    topic: KafkaString(String::from("some-topic").into()),
                    partitions: Some(vec![
                        ListOffsetsRequestV4_partitions{
                            partition: 0,
                            current_leader_epoch: 1,
                            timestamp: 2,
                        },
                        ListOffsetsRequestV4_partitions{
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
    fn test_ListOffsetsResponseV4() {
        write_then_read_eq(ListOffsetsResponseV4{
            throttle_time_ms: 0,
            responses: Some(vec![
                ListOffsetsResponseV4_responses{
                    topic: KafkaString(String::from("some-topic").into()),
                    partition_responses: Some(vec![
                        ListOffsetsResponseV4_partition_responses{
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
