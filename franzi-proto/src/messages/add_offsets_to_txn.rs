kafka_message!(
    "AddOffsetsToTxn Request (Version: 0) => transactional_id producer_id producer_epoch group_id
  transactional_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  group_id => STRING

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
group_id	The unique group identifier"
);

kafka_message!(
    "AddOffsetsToTxn Request (Version: 1) => transactional_id producer_id producer_epoch group_id
  transactional_id => STRING
  producer_id => INT64
  producer_epoch => INT16
  group_id => STRING

Field 	Description
transactional_id	The transactional id corresponding to the transaction.
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
group_id	The unique group identifier"
);

kafka_message!("AddOffsetsToTxn Response (Version: 0) => throttle_time_ms error_code
  throttle_time_ms => INT32
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code");

kafka_message!("AddOffsetsToTxn Response (Version: 1) => throttle_time_ms error_code
  throttle_time_ms => INT32
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code");
