kafka_message!(
    "OffsetCommit Request (Version: 0) => group_id [topics]
  group_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 1) => group_id generation_id member_id [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset timestamp metadata
      partition => INT32
      offset => INT64
      timestamp => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
timestamp	Timestamp of the commit
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 2) => group_id generation_id member_id retention_time [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  retention_time => INT64
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
retention_time	Time period in ms to retain the offset.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 3) => group_id generation_id member_id retention_time [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  retention_time => INT64
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
retention_time	Time period in ms to retain the offset.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 4) => group_id generation_id member_id retention_time [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  retention_time => INT64
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
retention_time	Time period in ms to retain the offset.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 5) => group_id generation_id member_id [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Request (Version: 6) => group_id generation_id member_id [topics]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset leader_epoch metadata
      partition => INT32
      offset => INT64
      leader_epoch => INT32
      metadata => NULLABLE_STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if
joining for the first time.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
leader_epoch	The leader epoch, if provided is derived from the last
consumed record. This is used by the consumer to check for log
truncation and to ensure partition metadata is up to date following a
group rebalance.
metadata	Any associated metadata the client wants to keep."
);

kafka_message!(
    "OffsetCommit Response (Version: 0) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 1) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 2) => [responses]
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 3) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 4) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 5) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "OffsetCommit Response (Version: 6) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
responses	Responses by topic for committed partitions
topic	Name of topic
partition_responses	Responses for committed partitions
partition	Topic partition id
error_code	Response error code"
);
