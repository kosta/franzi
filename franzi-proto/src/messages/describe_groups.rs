kafka_message!("DescribeGroups Request (Version: 0) => [group_ids]
  group_ids => STRING

Field 	Description
group_ids	List of groupIds to request metadata for (an empty groupId array will return empty group metadata).");

kafka_message!("DescribeGroups Request (Version: 1) => [group_ids]
  group_ids => STRING

Field 	Description
group_ids	List of groupIds to request metadata for (an empty groupId array will return empty group metadata).");

kafka_message!("DescribeGroups Request (Version: 2) => [group_ids]
  group_ids => STRING

Field 	Description
group_ids	List of groupIds to request metadata for (an empty groupId array will return empty group metadata).");

kafka_message!("DescribeGroups Response (Version: 0) => [groups]
  groups => error_code group_id state protocol_type protocol [members]
    error_code => INT16
    group_id => STRING
    state => STRING
    protocol_type => STRING
    protocol => STRING
    members => member_id client_id client_host member_metadata member_assignment
      member_id => STRING
      client_id => STRING
      client_host => STRING
      member_metadata => BYTES
      member_assignment => BYTES

Field 	Description
groups	null
error_code	Response error code
group_id	The unique group identifier
state	The current state of the group (one of: Dead, Stable, CompletingRebalance, PreparingRebalance, or empty if there is no active group)
protocol_type	The current group protocol type (will be empty if there is no active group)
protocol	The current group protocol (only provided if the group is Stable)
members	Current group members (only provided if the group is not Dead)
member_id	The member id assigned by the group coordinator or null if joining for the first time.
client_id	The client id used in the member's latest join group request
client_host	The client host used in the request session corresponding to the member's join group.
member_metadata	The metadata corresponding to the current group protocol in use (will only be present if the group is stable).
member_assignment	The current assignment provided by the group leader (will only be present if the group is stable).");

kafka_message!("DescribeGroups Response (Version: 1) => throttle_time_ms [groups]
  throttle_time_ms => INT32
  groups => error_code group_id state protocol_type protocol [members]
    error_code => INT16
    group_id => STRING
    state => STRING
    protocol_type => STRING
    protocol => STRING
    members => member_id client_id client_host member_metadata member_assignment
      member_id => STRING
      client_id => STRING
      client_host => STRING
      member_metadata => BYTES
      member_assignment => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
groups	null
error_code	Response error code
group_id	The unique group identifier
state	The current state of the group (one of: Dead, Stable, CompletingRebalance, PreparingRebalance, or empty if there is no active group)
protocol_type	The current group protocol type (will be empty if there is no active group)
protocol	The current group protocol (only provided if the group is Stable)
members	Current group members (only provided if the group is not Dead)
member_id	The member id assigned by the group coordinator or null if joining for the first time.
client_id	The client id used in the member's latest join group request
client_host	The client host used in the request session corresponding to the member's join group.
member_metadata	The metadata corresponding to the current group protocol in use (will only be present if the group is stable).
member_assignment	The current assignment provided by the group leader (will only be present if the group is stable).");

kafka_message!("DescribeGroups Response (Version: 2) => throttle_time_ms [groups]
  throttle_time_ms => INT32
  groups => error_code group_id state protocol_type protocol [members]
    error_code => INT16
    group_id => STRING
    state => STRING
    protocol_type => STRING
    protocol => STRING
    members => member_id client_id client_host member_metadata member_assignment
      member_id => STRING
      client_id => STRING
      client_host => STRING
      member_metadata => BYTES
      member_assignment => BYTES

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
groups	null
error_code	Response error code
group_id	The unique group identifier
state	The current state of the group (one of: Dead, Stable, CompletingRebalance, PreparingRebalance, or empty if there is no active group)
protocol_type	The current group protocol type (will be empty if there is no active group)
protocol	The current group protocol (only provided if the group is Stable)
members	Current group members (only provided if the group is not Dead)
member_id	The member id assigned by the group coordinator or null if joining for the first time.
client_id	The client id used in the member's latest join group request
client_host	The client host used in the request session corresponding to the member's join group.
member_metadata	The metadata corresponding to the current group protocol in use (will only be present if the group is stable).
member_assignment	The current assignment provided by the group leader (will only be present if the group is stable).");
