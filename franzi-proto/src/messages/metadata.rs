kafka_message!(
    "Metadata Request (Version: 0) => [topics]
  topics => STRING

Field 	Description
topics	An array of topics to fetch metadata for. If no topics are
specified fetch metadata for all topics."
);

kafka_message!(
    "Metadata Request (Version: 1) => [topics]
  topics => STRING

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics."
);

kafka_message!(
    "Metadata Request (Version: 2) => [topics]
  topics => STRING

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics."
);

kafka_message!(
    "Metadata Request (Version: 3) => [topics]
  topics => STRING

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics."
);

kafka_message!(
    "Metadata Request (Version: 4) => [topics] allow_auto_topic_creation
  topics => STRING
  allow_auto_topic_creation => BOOLEAN

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics.
allow_auto_topic_creation	If this and the broker config
|auto.create.topics.enable| are true, topics that don't exist will be
created by the broker. Otherwise, no topics will be created by the broker."
);

kafka_message!(
    "Metadata Request (Version: 5) => [topics] allow_auto_topic_creation
  topics => STRING
  allow_auto_topic_creation => BOOLEAN

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics.
allow_auto_topic_creation	If this and the broker config
|auto.create.topics.enable| are true, topics that don't exist will be
created by the broker. Otherwise, no topics will be created by the broker."
);

kafka_message!(
    "Metadata Request (Version: 6) => [topics] allow_auto_topic_creation
  topics => STRING
  allow_auto_topic_creation => BOOLEAN

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics.
allow_auto_topic_creation	If this and the broker config
|auto.create.topics.enable| are true, topics that don't exist will be
created by the broker. Otherwise, no topics will be created by the broker."
);

kafka_message!(
    "Metadata Request (Version: 7) => [topics] allow_auto_topic_creation
  topics => STRING
  allow_auto_topic_creation => BOOLEAN

Field 	Description
topics	An array of topics to fetch metadata for. If the topics array is
null fetch metadata for all topics.
allow_auto_topic_creation	If this and the broker config
|auto.create.topics.enable| are true, topics that don't exist will be
created by the broker. Otherwise, no topics will be created by the broker."
);

kafka_message!(
    "Metadata Response (Version: 0) => [brokers] [topic_metadata]
  brokers => node_id host port
    node_id => INT32
    host => STRING
    port => INT32
  topic_metadata => error_code topic [partition_metadata]
    error_code => INT16
    topic => STRING
    partition_metadata => error_code partition leader [replicas] [isr]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32

Field 	Description
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition."
);

kafka_message!(
    "Metadata Response (Version: 1) => [brokers] controller_id [topic_metadata]
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32

Field 	Description
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition."
);

kafka_message!(
    "Metadata Response (Version: 2) => [brokers] cluster_id controller_id [topic_metadata]
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32

Field 	Description
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition."
);

kafka_message!("Metadata Response (Version: 3) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
  throttle_time_ms => INT32
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition.");

kafka_message!("Metadata Response (Version: 4) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
  throttle_time_ms => INT32
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition.");

kafka_message!("Metadata Response (Version: 5) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
  throttle_time_ms => INT32
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr] [offline_replicas]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32
      offline_replicas => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition.
offline_replicas	The set of offline replicas of this partition.");

kafka_message!("Metadata Response (Version: 6) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
  throttle_time_ms => INT32
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader [replicas] [isr] [offline_replicas]
      error_code => INT16
      partition => INT32
      leader => INT32
      replicas => INT32
      isr => INT32
      offline_replicas => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition.
offline_replicas	The set of offline replicas of this partition.");

kafka_message!("Metadata Response (Version: 7) => throttle_time_ms [brokers] cluster_id controller_id [topic_metadata]
  throttle_time_ms => INT32
  brokers => node_id host port rack
    node_id => INT32
    host => STRING
    port => INT32
    rack => NULLABLE_STRING
  cluster_id => NULLABLE_STRING
  controller_id => INT32
  topic_metadata => error_code topic is_internal [partition_metadata]
    error_code => INT16
    topic => STRING
    is_internal => BOOLEAN
    partition_metadata => error_code partition leader leader_epoch [replicas] [isr] [offline_replicas]
      error_code => INT16
      partition => INT32
      leader => INT32
      leader_epoch => INT32
      replicas => INT32
      isr => INT32
      offline_replicas => INT32

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
brokers	Host and port information for all brokers.
node_id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.
rack	The rack of the broker.
cluster_id	The cluster id that this broker belongs to.
controller_id	The broker id of the controller broker.
topic_metadata	Metadata for requested topics
error_code	Response error code
topic	Name of topic
is_internal	Indicates if the topic is considered a Kafka internal topic
partition_metadata	Metadata for each partition of the topic.
error_code	Response error code
partition	Topic partition id
leader	The id of the broker acting as leader for this partition.
leader_epoch	The leader epoch
replicas	The set of all nodes that host this partition.
isr	The set of nodes that are in sync with the leader for this partition.
offline_replicas	The set of offline replicas of this partition.");
