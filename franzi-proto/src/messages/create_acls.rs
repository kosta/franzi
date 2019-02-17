kafka_message!(
    "CreateAcls Request (Version: 0) => [creations]
  creations => resource_type resource_name principal host operation permission_type
    resource_type => INT8
    resource_name => STRING
    principal => STRING
    host => STRING
    operation => INT8
    permission_type => INT8

Field 	Description
creations	null
resource_type	The resource type
resource_name	The resource name
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type"
);

kafka_message!("CreateAcls Request (Version: 1) => [creations]
  creations => resource_type resource_name resource_pattten_type principal host operation permission_type
    resource_type => INT8
    resource_name => STRING
    resource_pattten_type => INT8
    principal => STRING
    host => STRING
    operation => INT8
    permission_type => INT8

Field 	Description
creations	null
resource_type	The resource type
resource_name	The resource name
resource_pattten_type	The resource pattern type
principal	The ACL principal
host	The ACL host
operation	The ACL operation
permission_type	The ACL permission type");

kafka_message!("CreateAcls Response (Version: 0) => throttle_time_ms [creation_responses]
  throttle_time_ms => INT32
  creation_responses => error_code error_message
    error_code => INT16
    error_message => NULLABLE_STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
creation_responses	null
error_code	Response error code
error_message	Response error message");

kafka_message!("CreateAcls Response (Version: 1) => throttle_time_ms [creation_responses]
  throttle_time_ms => INT32
  creation_responses => error_code error_message
    error_code => INT16
    error_message => NULLABLE_STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
creation_responses	null
error_code	Response error code
error_message	Response error message");
