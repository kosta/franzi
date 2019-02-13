kafka_message!("ListGroups Request (Version: 0) =>

Field 	Description");

kafka_message!("ListGroups Request (Version: 1) =>

Field 	Description");

kafka_message!("ListGroups Request (Version: 2) =>

Field 	Description");

kafka_message!("ListGroups Response (Version: 0) => error_code [groups]
  error_code => INT16
  groups => group_id protocol_type
    group_id => STRING
    protocol_type => STRING

Field 	Description
error_code	Response error code
groups	null
group_id	The unique group identifier
protocol_type	null");

kafka_message!("ListGroups Response (Version: 1) => throttle_time_ms error_code [groups]
  throttle_time_ms => INT32
  error_code => INT16
  groups => group_id protocol_type
    group_id => STRING
    protocol_type => STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
groups	null
group_id	The unique group identifier
protocol_type	null");

kafka_message!("ListGroups Response (Version: 2) => throttle_time_ms error_code [groups]
  throttle_time_ms => INT32
  error_code => INT16
  groups => group_id protocol_type
    group_id => STRING
    protocol_type => STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
groups	null
group_id	The unique group identifier
protocol_type	null");
