kafka_message!(
    "DeleteAcls Request (Version: 0) => [filters]
  filters => resource_type resource_name principal host operation permission_type
    resource_type => INT8
    resource_name => NULLABLE_STRING
    principal => NULLABLE_STRING
    host => NULLABLE_STRING
    operation => INT8
    permission_type => INT8

Field 	Description
filters	null
resource_type	The resource type
resource_name	The resource name filter
principal	The ACL principal filter
host	The ACL host filter
operation	The ACL operation
permission_type	The ACL permission type"
);

kafka_message!("DeleteAcls Request (Version: 1) => [filters]
  filters => resource_type resource_name resource_pattern_type_filter principal host operation permission_type
    resource_type => INT8
    resource_name => NULLABLE_STRING
    resource_pattern_type_filter => INT8
    principal => NULLABLE_STRING
    host => NULLABLE_STRING
    operation => INT8
    permission_type => INT8

Field 	Description
filters	null
resource_type	The resource type
resource_name	The resource name filter
resource_pattern_type_filter	The resource pattern type filter
principal	The ACL principal filter
host	The ACL host filter
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("DeleteAcls Response (Version: 0) => throttle_time_ms [filter_responses]
  throttle_time_ms => INT32
  filter_responses => error_code error_message [matching_acls]
    error_code => INT16
    error_message => NULLABLE_STRING
    matching_acls => error_code error_message resource_type resource_name principal host operation permission_type
      error_code => INT16
      error_message => NULLABLE_STRING
      resource_type => INT8
      resource_name => STRING
      principal => STRING
      host => STRING
      operation => INT8
      permission_type => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
filter_responses	null
error_code	Response error code
error_message	Response error message
matching_acls	The matching ACLs
error_code	Response error code
error_message	Response error message
resource_type	The resource type
resource_name	The resource name
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("DeleteAcls Response (Version: 1) => throttle_time_ms [filter_responses]
  throttle_time_ms => INT32
  filter_responses => error_code error_message [matching_acls]
    error_code => INT16
    error_message => NULLABLE_STRING
    matching_acls => error_code error_message resource_type resource_name resource_pattten_type principal host operation permission_type
      error_code => INT16
      error_message => NULLABLE_STRING
      resource_type => INT8
      resource_name => STRING
      resource_pattten_type => INT8
      principal => STRING
      host => STRING
      operation => INT8
      permission_type => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
filter_responses	null
error_code	Response error code
error_message	Response error message
matching_acls	The matching ACLs
error_code	Response error code
error_message	Response error message
resource_type	The resource type
resource_name	The resource name
resource_pattten_type	The resource pattern type
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type");
