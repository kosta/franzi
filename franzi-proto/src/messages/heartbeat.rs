kafka_message!(
    "Heartbeat Request (Version: 0) => group_id generation_id member_id
  group_id => STRING
  generation_id => INT32
  member_id => STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time."
);

kafka_message!(
    "Heartbeat Request (Version: 1) => group_id generation_id member_id
  group_id => STRING
  generation_id => INT32
  member_id => STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time."
);

kafka_message!(
    "Heartbeat Request (Version: 2) => group_id generation_id member_id
  group_id => STRING
  generation_id => INT32
  member_id => STRING

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time."
);

kafka_message!(
    "Heartbeat Response (Version: 0) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code"
);

kafka_message!("Heartbeat Response (Version: 1) => throttle_time_ms error_code
  throttle_time_ms => INT32
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code");

kafka_message!("Heartbeat Response (Version: 2) => throttle_time_ms error_code
  throttle_time_ms => INT32
  error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code");
