kafka_message!("InitProducerId Request (Version: 0) => transactional_id transaction_timeout_ms
  transactional_id => NULLABLE_STRING
  transaction_timeout_ms => INT32

Field 	Description
transactional_id	The transactional id or null if the producer is not transactional
transaction_timeout_ms	The time in ms to wait for before aborting idle transactions sent by this producer.");

kafka_message!("InitProducerId Request (Version: 1) => transactional_id transaction_timeout_ms
  transactional_id => NULLABLE_STRING
  transaction_timeout_ms => INT32

Field 	Description
transactional_id	The transactional id or null if the producer is not transactional
transaction_timeout_ms	The time in ms to wait for before aborting idle transactions sent by this producer.");

kafka_message!("InitProducerId Response (Version: 0) => throttle_time_ms error_code producer_id producer_epoch
  throttle_time_ms => INT32
  error_code => INT16
  producer_id => INT64
  producer_epoch => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.");

kafka_message!("InitProducerId Response (Version: 1) => throttle_time_ms error_code producer_id producer_epoch
  throttle_time_ms => INT32
  error_code => INT16
  producer_id => INT64
  producer_epoch => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.");
