kafka_message!("AddPartitionsToTxn Request (Version: 0) => transactional_id producer_id producer_epoch [topics]
  transactional_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  topics => topic [partitions]
    topic => STRING
    partitions => INT32

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
topics	The partitions to add to the transaction.
topic	Name of topic
partitions	null");

kafka_message!("AddPartitionsToTxn Request (Version: 1) => transactional_id producer_id producer_epoch [topics]
  transactional_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  topics => topic [partitions]
    topic => STRING
    partitions => INT32

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
topics	The partitions to add to the transaction.
topic	Name of topic
partitions	null");

kafka_message!("AddPartitionsToTxn Response (Version: 0) => throttle_time_ms [errors]
  throttle_time_ms => INT32
  errors => topic [partition_errors]
    topic => STRING
    partition_errors => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
errors	null
topic	Name of topic
partition_errors	null
partition	Topic partition id
error_code	Response error code");

kafka_message!("AddPartitionsToTxn Response (Version: 1) => throttle_time_ms [errors]
  throttle_time_ms => INT32
  errors => topic [partition_errors]
    topic => STRING
    partition_errors => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
errors	null
topic	Name of topic
partition_errors	null
partition	Topic partition id
error_code	Response error code");
