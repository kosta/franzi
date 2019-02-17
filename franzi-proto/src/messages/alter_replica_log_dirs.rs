kafka_message!("AlterReplicaLogDirs Request (Version: 0) => [log_dirs]
  log_dirs => log_dir [topics]
    log_dir => STRING
    topics => topic [partitions]
      topic => STRING
      partitions => INT32

Field 	Description
log_dirs	null
log_dir	The absolute log directory path.
topics	null
topic	Name of topic
partitions	List of partition ids of the topic.");

kafka_message!("AlterReplicaLogDirs Request (Version: 1) => [log_dirs]
  log_dirs => log_dir [topics]
    log_dir => STRING
    topics => topic [partitions]
      topic => STRING
      partitions => INT32

Field 	Description
log_dirs	null
log_dir	The absolute log directory path.
topics	null
topic	Name of topic
partitions	List of partition ids of the topic.");

kafka_message!("AlterReplicaLogDirs Response (Version: 0) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
error_code	Response error code");

kafka_message!("AlterReplicaLogDirs Response (Version: 1) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition error_code
      partition => INT32
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
error_code	Response error code");
