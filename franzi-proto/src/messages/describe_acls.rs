kafka_message!("DescribeAcls Request (Version: 0) => resource_type resource_name principal host operation permission_type
  resource_type => INT8
  resource_name => NULLABLE_STRING
  principal => NULLABLE_STRING
  host => NULLABLE_STRING
  operation => INT8
  permission_type => INT8

Field 	Description
resource_type	The resource type
resource_name	The resource name filter
principal	The ACL principal filter
host	The ACL host filter
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("DescribeAcls Request (Version: 1) => resource_type resource_name resource_pattern_type_filter principal host operation permission_type
  resource_type => INT8
  resource_name => NULLABLE_STRING
  resource_pattern_type_filter => INT8
  principal => NULLABLE_STRING
  host => NULLABLE_STRING
  operation => INT8
  permission_type => INT8

Field 	Description
resource_type	The resource type
resource_name	The resource name filter
resource_pattern_type_filter	The resource pattern type filter
principal	The ACL principal filter
host	The ACL host filter
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("DescribeAcls Response (Version: 0) => throttle_time_ms error_code error_message [resources]
  throttle_time_ms => INT32
  error_code => INT16
  error_message => NULLABLE_STRING
  resources => resource_type resource_name [acls]
    resource_type => INT8
    resource_name => STRING
    acls => principal host operation permission_type
      principal => STRING
      host => STRING
      operation => INT8
      permission_type => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
error_message	Response error message
resources	The resources and their associated ACLs.
resource_type	The resource type
resource_name	The resource name
acls	null
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("DescribeAcls Response (Version: 1) => throttle_time_ms error_code error_message [resources]
  throttle_time_ms => INT32
  error_code => INT16
  error_message => NULLABLE_STRING
  resources => resource_type resource_name resource_pattten_type [acls]
    resource_type => INT8
    resource_name => STRING
    resource_pattten_type => INT8
    acls => principal host operation permission_type
      principal => STRING
      host => STRING
      operation => INT8
      permission_type => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
error_code	Response error code
error_message	Response error message
resources	The resources and their associated ACLs.
resource_type	The resource type
resource_name	The resource name
resource_pattten_type	The resource pattern type
acls	null
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type");
