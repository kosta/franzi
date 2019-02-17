kafka_message!(
    "DescribeConfigs Request (Version: 0) => [resources]
  resources => resource_type resource_name [config_names]
    resource_type => INT8
    resource_name => STRING
    config_names => STRING

Field 	Description
resources	An array of config resources to be returned.
resource_type	null
resource_name	null
config_names	null"
);

kafka_message!(
    "DescribeConfigs Request (Version: 1) => [resources] include_synonyms
  resources => resource_type resource_name [config_names]
    resource_type => INT8
    resource_name => STRING
    config_names => STRING
  include_synonyms => BOOLEAN

Field 	Description
resources	An array of config resources to be returned.
resource_type	null
resource_name	null
config_names	null
include_synonyms	null"
);

kafka_message!(
    "DescribeConfigs Request (Version: 2) => [resources] include_synonyms
  resources => resource_type resource_name [config_names]
    resource_type => INT8
    resource_name => STRING
    config_names => STRING
  include_synonyms => BOOLEAN

Field 	Description
resources	An array of config resources to be returned.
resource_type	null
resource_name	null
config_names	null
include_synonyms	null"
);

kafka_message!("DescribeConfigs Response (Version: 0) => throttle_time_ms [resources]
  throttle_time_ms => INT32
  resources => error_code error_message resource_type resource_name [config_entries]
    error_code => INT16
    error_message => NULLABLE_STRING
    resource_type => INT8
    resource_name => STRING
    config_entries => config_name config_value read_only is_default is_sensitive
      config_name => STRING
      config_value => NULLABLE_STRING
      read_only => BOOLEAN
      is_default => BOOLEAN
      is_sensitive => BOOLEAN

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
resources	null
error_code	Response error code
error_message	Response error message
resource_type	null
resource_name	null
config_entries	null
config_name	null
config_value	null
read_only	null
is_default	null
is_sensitive	null");

kafka_message!("DescribeConfigs Response (Version: 1) => throttle_time_ms [resources]
  throttle_time_ms => INT32
  resources => error_code error_message resource_type resource_name [config_entries]
    error_code => INT16
    error_message => NULLABLE_STRING
    resource_type => INT8
    resource_name => STRING
    config_entries => config_name config_value read_only config_source is_sensitive [config_synonyms]
      config_name => STRING
      config_value => NULLABLE_STRING
      read_only => BOOLEAN
      config_source => INT8
      is_sensitive => BOOLEAN
      config_synonyms => config_name config_value config_source
        config_name => STRING
        config_value => NULLABLE_STRING
        config_source => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
resources	null
error_code	Response error code
error_message	Response error message
resource_type	null
resource_name	null
config_entries	null
config_name	null
config_value	null
read_only	null
config_source	null
is_sensitive	null
config_synonyms	null
config_name	null
config_value	null
config_source	null");

kafka_message!("DescribeConfigs Response (Version: 2) => throttle_time_ms [resources]
  throttle_time_ms => INT32
  resources => error_code error_message resource_type resource_name [config_entries]
    error_code => INT16
    error_message => NULLABLE_STRING
    resource_type => INT8
    resource_name => STRING
    config_entries => config_name config_value read_only config_source is_sensitive [config_synonyms]
      config_name => STRING
      config_value => NULLABLE_STRING
      read_only => BOOLEAN
      config_source => INT8
      is_sensitive => BOOLEAN
      config_synonyms => config_name config_value config_source
        config_name => STRING
        config_value => NULLABLE_STRING
        config_source => INT8

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
resources	null
error_code	Response error code
error_message	Response error message
resource_type	null
resource_name	null
config_entries	null
config_name	null
config_value	null
read_only	null
config_source	null
is_sensitive	null
config_synonyms	null
config_name	null
config_value	null
config_source	null");
