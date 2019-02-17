kafka_message!(
    "AlterConfigs Request (Version: 0) => [resources] validate_only
  resources => resource_type resource_name [config_entries]
    resource_type => INT8
    resource_name => STRING
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  validate_only => BOOLEAN

Field 	Description
resources	An array of resources to update with the provided configs.
resource_type	null
resource_name	null
config_entries	null
config_name	Configuration name
config_value	Configuration value
validate_only	null"
);

kafka_message!(
    "AlterConfigs Request (Version: 1) => [resources] validate_only
  resources => resource_type resource_name [config_entries]
    resource_type => INT8
    resource_name => STRING
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  validate_only => BOOLEAN

Field 	Description
resources	An array of resources to update with the provided configs.
resource_type	null
resource_name	null
config_entries	null
config_name	Configuration name
config_value	Configuration value
validate_only	null"
);

kafka_message!("AlterConfigs Response (Version: 0) => throttle_time_ms [resources]
  throttle_time_ms => INT32
  resources => error_code error_message resource_type resource_name
    error_code => INT16
    error_message => NULLABLE_STRING
    resource_type => INT8
    resource_name => STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
resources	null
error_code	Response error code
error_message	Response error message
resource_type	null
resource_name	null");

kafka_message!("AlterConfigs Response (Version: 1) => throttle_time_ms [resources]
  throttle_time_ms => INT32
  resources => error_code error_message resource_type resource_name
    error_code => INT16
    error_message => NULLABLE_STRING
    resource_type => INT8
    resource_name => STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
resources	null
error_code	Response error code
error_message	Response error message
resource_type	null
resource_name	null");
