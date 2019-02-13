kafka_message!("CreateTopics Request (Version: 0) => [create_topic_requests] timeout
  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries]
    topic => STRING
    num_partitions => INT32
    replication_factor => INT16
    replica_assignment => partition [replicas]
      partition => INT32
      replicas => INT32
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  timeout => INT32

Field 	Description
create_topic_requests	An array of single topic creation requests. Can not have multiple entries for the same topic.
topic	Name of topic
num_partitions	Number of partitions to be created. -1 indicates unset.
replication_factor	Replication factor for the topic. -1 indicates unset.
replica_assignment	Replica assignment among kafka brokers for this topic partitions. If this is set num_partitions and replication_factor must be unset.
partition	Topic partition id
replicas	The set of all nodes that should host this partition. The first replica in the list is the preferred leader.
config_entries	Topic level configuration for topic to be set.
config_name	Configuration name
config_value	Configuration value
timeout	The time in ms to wait for a topic to be completely created on the controller node. Values <= 0 will trigger topic creation and return immediately");

kafka_message!("CreateTopics Request (Version: 1) => [create_topic_requests] timeout validate_only
  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries]
    topic => STRING
    num_partitions => INT32
    replication_factor => INT16
    replica_assignment => partition [replicas]
      partition => INT32
      replicas => INT32
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  timeout => INT32
  validate_only => BOOLEAN

Field 	Description
create_topic_requests	An array of single topic creation requests. Can not have multiple entries for the same topic.
topic	Name of topic
num_partitions	Number of partitions to be created. -1 indicates unset.
replication_factor	Replication factor for the topic. -1 indicates unset.
replica_assignment	Replica assignment among kafka brokers for this topic partitions. If this is set num_partitions and replication_factor must be unset.
partition	Topic partition id
replicas	The set of all nodes that should host this partition. The first replica in the list is the preferred leader.
config_entries	Topic level configuration for topic to be set.
config_name	Configuration name
config_value	Configuration value
timeout	The time in ms to wait for a topic to be completely created on the controller node. Values <= 0 will trigger topic creation and return immediately
validate_only	If this is true, the request will be validated, but the
topic won't be created.");

kafka_message!("CreateTopics Request (Version: 2) => [create_topic_requests] timeout validate_only
  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries]
    topic => STRING
    num_partitions => INT32
    replication_factor => INT16
    replica_assignment => partition [replicas]
      partition => INT32
      replicas => INT32
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  timeout => INT32
  validate_only => BOOLEAN

Field 	Description
create_topic_requests	An array of single topic creation requests. Can not have multiple entries for the same topic.
topic	Name of topic
num_partitions	Number of partitions to be created. -1 indicates unset.
replication_factor	Replication factor for the topic. -1 indicates unset.
replica_assignment	Replica assignment among kafka brokers for this topic partitions. If this is set num_partitions and replication_factor must be unset.
partition	Topic partition id
replicas	The set of all nodes that should host this partition. The first replica in the list is the preferred leader.
config_entries	Topic level configuration for topic to be set.
config_name	Configuration name
config_value	Configuration value
timeout	The time in ms to wait for a topic to be completely created on the controller node. Values <= 0 will trigger topic creation and return immediately
validate_only	If this is true, the request will be validated, but the
topic won't be created.");

kafka_message!("CreateTopics Request (Version: 3) => [create_topic_requests] timeout validate_only
  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries]
    topic => STRING
    num_partitions => INT32
    replication_factor => INT16
    replica_assignment => partition [replicas]
      partition => INT32
      replicas => INT32
    config_entries => config_name config_value
      config_name => STRING
      config_value => NULLABLE_STRING
  timeout => INT32
  validate_only => BOOLEAN

Field 	Description
create_topic_requests	An array of single topic creation requests. Can not have multiple entries for the same topic.
topic	Name of topic
num_partitions	Number of partitions to be created. -1 indicates unset.
replication_factor	Replication factor for the topic. -1 indicates unset.
replica_assignment	Replica assignment among kafka brokers for this topic partitions. If this is set num_partitions and replication_factor must be unset.
partition	Topic partition id
replicas	The set of all nodes that should host this partition. The first replica in the list is the preferred leader.
config_entries	Topic level configuration for topic to be set.
config_name	Configuration name
config_value	Configuration value
timeout	The time in ms to wait for a topic to be completely created on the controller node. Values <= 0 will trigger topic creation and return immediately
validate_only	If this is true, the request will be validated, but the topic won't be created.");

kafka_message!("CreateTopics Response (Version: 0) => [topic_errors]
  topic_errors => topic error_code
    topic => STRING
    error_code => INT16

Field 	Description
topic_errors	An array of per topic error codes.
topic	Name of topic
error_code	Response error code");

kafka_message!("CreateTopics Response (Version: 1) => [topic_errors]
  topic_errors => topic error_code error_message
    topic => STRING
    error_code => INT16
    error_message => NULLABLE_STRING

Field 	Description
topic_errors	An array of per topic errors.
topic	Name of topic
error_code	Response error code
error_message	Response error message");

kafka_message!("CreateTopics Response (Version: 2) => throttle_time_ms [topic_errors]
  throttle_time_ms => INT32
  topic_errors => topic error_code error_message
    topic => STRING
    error_code => INT16
    error_message => NULLABLE_STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topic_errors	An array of per topic errors.
topic	Name of topic
error_code	Response error code
error_message	Response error message");

kafka_message!("CreateTopics Response (Version: 3) => throttle_time_ms [topic_errors]
  throttle_time_ms => INT32
  topic_errors => topic error_code error_message
    topic => STRING
    error_code => INT16
    error_message => NULLABLE_STRING

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topic_errors	An array of per topic errors.
topic	Name of topic
error_code	Response error code
error_message	Response error message");
