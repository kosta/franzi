kafka_message!("OffsetForLeaderEpoch Request (Version: 0) => [topics]
  topics => topic [partitions]
    topic => STRING
    partitions => partition leader_epoch
      partition => INT32
      leader_epoch => INT32

Field 	Description
topics	An array of topics to get epochs for
topic	Name of topic
partitions	An array of partitions to get epochs for
partition	Topic partition id
leader_epoch	The epoch to lookup an offset for.");

kafka_message!("OffsetForLeaderEpoch Request (Version: 1) => [topics]
  topics => topic [partitions]
    topic => STRING
    partitions => partition leader_epoch
      partition => INT32
      leader_epoch => INT32

Field 	Description
topics	An array of topics to get epochs for
topic	Name of topic
partitions	An array of partitions to get epochs for
partition	Topic partition id
leader_epoch	The epoch to lookup an offset for.");

kafka_message!("OffsetForLeaderEpoch Request (Version: 2) => [topics]
  topics => topic [partitions]
    topic => STRING
    partitions => partition current_leader_epoch leader_epoch
      partition => INT32
      current_leader_epoch => INT32
      leader_epoch => INT32

Field 	Description
topics	An array of topics to get epochs for
topic	Name of topic
partitions	An array of partitions to get epochs for
partition	Topic partition id
current_leader_epoch	The current leader epoch, if provided, is used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
leader_epoch	The epoch to lookup an offset for.");

kafka_message!("OffsetForLeaderEpoch Response (Version: 0) => [topics]
  topics => topic [partitions]
    topic => STRING
    partitions => error_code partition end_offset
      error_code => INT16
      partition => INT32
      end_offset => INT64

Field 	Description
topics	An array of topics for which we have leader offsets for some requested partition leader epoch
topic	Name of topic
partitions	An array of offsets by partition
error_code	Response error code
partition	Topic partition id
end_offset	The end offset");

kafka_message!("OffsetForLeaderEpoch Response (Version: 1) => [topics]
  topics => topic [partitions]
    topic => STRING
    partitions => error_code partition leader_epoch end_offset
      error_code => INT16
      partition => INT32
      leader_epoch => INT32
      end_offset => INT64

Field 	Description
topics	An array of topics for which we have leader offsets for some requested partition leader epoch
topic	Name of topic
partitions	An array of offsets by partition
error_code	Response error code
partition	Topic partition id
leader_epoch	The leader epoch
end_offset	The end offset");

kafka_message!("OffsetForLeaderEpoch Response (Version: 2) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => error_code partition leader_epoch end_offset
      error_code => INT16
      partition => INT32
      leader_epoch => INT32
      end_offset => INT64

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	An array of topics for which we have leader offsets for some requested partition leader epoch
topic	Name of topic
partitions	An array of offsets by partition
error_code	Response error code
partition	Topic partition id
leader_epoch	The leader epoch
end_offset	The end offset");
