use franzi_macros::kafka_message;

kafka_message!(
    "FindCoordinator Request (Version: 2) => coordinator_key coordinator_type
    coordinator_key => STRING
    coordinator_type => INT8

Field Description
coordinator_key	Id to use for finding the coordinator (for groups, this is the groupId, for transactional producers, this is the transactional id)
coordinator_type	The type of coordinator to find (0 = group, 1 = transaction)"
);

kafka_message!(
  "FindCoordinator Response (Version: 2) => throttle_time_ms error_code error_message coordinator
  throttle_time_ms => INT32
  error_code => INT16
  error_message => NULLABLE_STRING
  coordinator => node_id host port
    node_id => INT32
    host => STRING
    port => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
error_message	Response error message
coordinator	Host and port information for the coordinator
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests."
);