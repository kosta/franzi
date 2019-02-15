kafka_message!("TxnOffsetCommit Request (Version: 0) => transactional_id group_id producer_id producer_epoch [topics]
  transactional_id => STRING
  group_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
group_id	The unique group identifier
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.");

kafka_message!("TxnOffsetCommit Request (Version: 1) => transactional_id group_id producer_id producer_epoch [topics]
  transactional_id => STRING
  group_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset metadata
      partition => INT32
      offset => INT64
      metadata => NULLABLE_STRING

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
group_id	The unique group identifier
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
metadata	Any associated metadata the client wants to keep.");

kafka_message!("TxnOffsetCommit Request (Version: 2) => transactional_id group_id producer_id producer_epoch [topics]
  transactional_id => STRING
  group_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset leader_epoch metadata
      partition => INT32
      offset => INT64
      leader_epoch => INT32
      metadata => NULLABLE_STRING

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
group_id	The unique group identifier
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
topics	Topics to commit offsets
topic	Name of topic
partitions	Partitions to commit offsets
partition	Topic partition id
offset	Message offset to be committed
leader_epoch	The leader epoch, if provided is derived from the last consumed record. This is used by the consumer to check for log truncation and to ensure partition metadata is up to date following a group rebalance.
metadata	Any associated metadata the client wants to keep.");

kafka_message!("TxnOffsetCommit Response (Version: 0) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	Responses by topic for committed offsets
topic	Name of topic
partitions	Responses by partition for committed offsets
partition	Topic partition id
error_code	Response error code");

kafka_message!("TxnOffsetCommit Response (Version: 1) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	Responses by topic for committed offsets
topic	Name of topic
partitions	Responses by partition for committed offsets
partition	Topic partition id
error_code	Response error code");

kafka_message!("TxnOffsetCommit Response (Version: 2) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	Responses by topic for committed offsets
topic	Name of topic
partitions	Responses by partition for committed offsets
partition	Topic partition id
error_code	Response error code");
