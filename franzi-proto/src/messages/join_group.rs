kafka_message!("JoinGroup Request (Version: 0) => group_id session_timeout member_id protocol_type [group_protocols]
  group_id => STRING
  session_timeout => INT32
  member_id => STRING
  protocol_type => STRING
  group_protocols => protocol_name protocol_metadata
    protocol_name => STRING
    protocol_metadata => BYTES

Field 	Description
group_id	The unique group identifier
session_timeout	The coordinator considers the consumer dead if it receives no heartbeat after this timeout in ms.
member_id	The member id assigned by the group coordinator or null if joining for the first time.
protocol_type	Unique name for class of protocols implemented by group
group_protocols	List of protocols that the member supports
protocol_name	null
protocol_metadata	null");

kafka_message!("JoinGroup Request (Version: 1) => group_id session_timeout rebalance_timeout member_id protocol_type [group_protocols]
  group_id => STRING
  session_timeout => INT32
  rebalance_timeout => INT32
  member_id => STRING
  protocol_type => STRING
  group_protocols => protocol_name protocol_metadata
    protocol_name => STRING
    protocol_metadata => BYTES

Field 	Description
group_id	The unique group identifier
session_timeout	The coordinator considers the consumer dead if it receives no heartbeat after this timeout in ms.
rebalance_timeout	The maximum time that the coordinator will wait for each member to rejoin when rebalancing the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
protocol_type	Unique name for class of protocols implemented by group
group_protocols	List of protocols that the member supports
protocol_name	null
protocol_metadata	null");

kafka_message!("JoinGroup Request (Version: 2) => group_id session_timeout rebalance_timeout member_id protocol_type [group_protocols]
  group_id => STRING
  session_timeout => INT32
  rebalance_timeout => INT32
  member_id => STRING
  protocol_type => STRING
  group_protocols => protocol_name protocol_metadata
    protocol_name => STRING
    protocol_metadata => BYTES

Field 	Description
group_id	The unique group identifier
session_timeout	The coordinator considers the consumer dead if it receives no heartbeat after this timeout in ms.
rebalance_timeout	The maximum time that the coordinator will wait for each member to rejoin when rebalancing the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
protocol_type	Unique name for class of protocols implemented by group
group_protocols	List of protocols that the member supports
protocol_name	null
protocol_metadata	null");

kafka_message!("JoinGroup Request (Version: 3) => group_id session_timeout rebalance_timeout member_id protocol_type [group_protocols]
  group_id => STRING
  session_timeout => INT32
  rebalance_timeout => INT32
  member_id => STRING
  protocol_type => STRING
  group_protocols => protocol_name protocol_metadata
    protocol_name => STRING
    protocol_metadata => BYTES

Field 	Description
group_id	The unique group identifier
session_timeout	The coordinator considers the consumer dead if it receives no heartbeat after this timeout in ms.
rebalance_timeout	The maximum time that the coordinator will wait for each member to rejoin when rebalancing the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
protocol_type	Unique name for class of protocols implemented by group
group_protocols	List of protocols that the member supports
protocol_name	null
protocol_metadata	null");

kafka_message!("JoinGroup Response (Version: 0) => error_code generation_id group_protocol leader_id member_id [members]
  error_code => INT16
  generation_id => INT32
  group_protocol => STRING
  leader_id => STRING
  member_id => STRING
  members => member_id member_metadata
    member_id => STRING
    member_metadata => BYTES

Field 	Description
error_code	Response error code
generation_id	The generation of the group.
group_protocol	The group protocol selected by the coordinator
leader_id	The leader of the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
members	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_metadata	null");

kafka_message!("JoinGroup Response (Version: 1) => error_code generation_id group_protocol leader_id member_id [members]
  error_code => INT16
  generation_id => INT32
  group_protocol => STRING
  leader_id => STRING
  member_id => STRING
  members => member_id member_metadata
    member_id => STRING
    member_metadata => BYTES

Field 	Description
error_code	Response error code
generation_id	The generation of the group.
group_protocol	The group protocol selected by the coordinator
leader_id	The leader of the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
members	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_metadata	null");

kafka_message!("JoinGroup Response (Version: 2) => throttle_time_ms error_code generation_id group_protocol leader_id member_id [members]
  throttle_time_ms => INT32
  error_code => INT16
  generation_id => INT32
  group_protocol => STRING
  leader_id => STRING
  member_id => STRING
  members => member_id member_metadata
    member_id => STRING
    member_metadata => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
generation_id	The generation of the group.
group_protocol	The group protocol selected by the coordinator
leader_id	The leader of the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
members	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_metadata	null");

kafka_message!("JoinGroup Response (Version: 3) => throttle_time_ms error_code generation_id group_protocol leader_id member_id [members]
  throttle_time_ms => INT32
  error_code => INT16
  generation_id => INT32
  group_protocol => STRING
  leader_id => STRING
  member_id => STRING
  members => member_id member_metadata
    member_id => STRING
    member_metadata => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
generation_id	The generation of the group.
group_protocol	The group protocol selected by the coordinator
leader_id	The leader of the group
member_id	The member id assigned by the group coordinator or null if joining for the first time.
members	null
member_id	The member id assigned by the group coordinator or null if joining for the first time.
member_metadata	null");
