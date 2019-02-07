kafka_message!(
    "OffsetFetch Request (Version: 0) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Request (Version: 1) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Request (Version: 2) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets. If the topic array is null fetch offsets for all topics.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Request (Version: 3) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets. If the topic array is null fetch offsets for all topics.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Request (Version: 4) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets. If the topic array is null fetch offsets for all topics.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Request (Version: 5) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition
      partition => INT32

Field 	Description
group_id	The unique group identifier
topics	Topics to fetch offsets. If the topic array is null fetch offsets for all topics.
topic	Name of topic
partitions	Partitions to fetch offsets.
partition	Topic partition id"
);

kafka_message!(
    "OffsetFetch Response (Version: 0) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset metadata error_code
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING
      error_code => INT16

Field 	Description
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.
error_code	Response error code"
);

kafka_message!(
    "OffsetFetch Response (Version: 1) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset metadata error_code
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING
      error_code => INT16

Field 	Description
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.
error_code	Response error code"
);

kafka_message!(
    "OffsetFetch Response (Version: 2) => [responses] error_code
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset metadata error_code
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING
      error_code => INT16
  error_code => INT16

Field 	Description
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.
error_code	Response error code
error_code	Response error code"
);

kafka_message!(
    "OffsetFetch Response (Version: 3) => throttle_time_ms [responses] error_code
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset metadata error_code
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING
      error_code => INT16
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.
error_code	Response error code
error_code	Response error code"
);

kafka_message!(
    "OffsetFetch Response (Version: 4) => throttle_time_ms [responses] error_code
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset metadata error_code
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING
      error_code => INT16
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.
error_code	Response error code
error_code	Response error code"
);

// TODO: Having two "differrent" error code fields breaks this macro!
kafka_message!(
    "OffsetFetch Response (Version: 5) => throttle_time_ms [responses] error_code
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition offset leader_epoch metadata error_code
      partition => INT32
      offset => INT64
      leader_epoch => INT32
      metadata => NULLABLE_STRING
      error_code => INT16
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
responses	Responses by topic for fetched offsets
topic	Name of topic
partition_responses	Responses by partition for fetched offsets
partition	Topic partition id
offset	Message offset to be committed
leader_epoch	The leader epoch, if provided is derived from the last consumed record. This is used by the consumer to check for log truncation and to ensure partition metadata is up to date following a group rebalance.
metadata	Any associated metadata the client wants to keep.
error_code	Response error code
error_code	Response error code"
);
