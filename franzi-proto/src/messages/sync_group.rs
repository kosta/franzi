kafka_message!(
    "SyncGroup Request (Version: 0) => group_id generation_id member_id [group_assignment]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  group_assignment => member_id member_assignment
    member_id => STRING
    member_assignment => BYTES

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time.
group_assignment	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_assignment	null"
);

kafka_message!(
    "SyncGroup Request (Version: 1) => group_id generation_id member_id [group_assignment]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  group_assignment => member_id member_assignment
    member_id => STRING
    member_assignment => BYTES

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time.
group_assignment	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_assignment	null"
);

kafka_message!(
    "SyncGroup Request (Version: 2) => group_id generation_id member_id [group_assignment]
  group_id => STRING
  generation_id => INT32
  member_id => STRING
  group_assignment => member_id member_assignment
    member_id => STRING
    member_assignment => BYTES

Field 	Description
group_id	The unique group identifier
generation_id	The generation of the group.
member_id	The member id assigned by the group coordinator or null if joining for the first time.
group_assignment	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_assignment	null"
);

kafka_message!(
    "SyncGroup Response (Version: 0) => error_code member_assignment
  error_code => INT16
  member_assignment => BYTES

Field 	Description
error_code	Response error code
member_assignment	null"
);

kafka_message!("SyncGroup Response (Version: 1) => throttle_time_ms error_code member_assignment
  throttle_time_ms => INT32
  error_code => INT16
  member_assignment => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
member_assignment	null");

kafka_message!("SyncGroup Response (Version: 2) => throttle_time_ms error_code member_assignment
  throttle_time_ms => INT32
  error_code => INT16
  member_assignment => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
member_assignment	null");
