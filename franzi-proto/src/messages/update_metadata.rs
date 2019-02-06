kafka_message!("UpdateMetadata Request (Version: 0) => controller_id controller_epoch [partition_states] [live_brokers]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas]
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
  live_brokers => id host port
    id => INT32
    host => STRING
    port => INT32

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
partition_states	null
topic	Name of topic
partition	Topic partition id
controller_epoch	The controller epoch.
leader	The broker id for the leader.
leader_epoch	The leader epoch.
isr	The in sync replica ids.
zk_version	The ZK version.
replicas	The replica ids.
live_brokers	null
id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.");

kafka_message!("UpdateMetadata Request (Version: 1) => controller_id controller_epoch [partition_states] [live_brokers]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas]
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
  live_brokers => id [end_points]
    id => INT32
    end_points => port host security_protocol_type
      port => INT32
      host => STRING
      security_protocol_type => INT16

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
partition_states	null
topic	Name of topic
partition	Topic partition id
controller_epoch	The controller epoch.
leader	The broker id for the leader.
leader_epoch	The leader epoch.
isr	The in sync replica ids.
zk_version	The ZK version.
replicas	The replica ids.
live_brokers	null
id	The broker id.
end_points	null
port	The port on which the broker accepts requests.
host	The hostname of the broker.
security_protocol_type	The security protocol type.");

kafka_message!("UpdateMetadata Request (Version: 2) => controller_id controller_epoch [partition_states] [live_brokers]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas]
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
  live_brokers => id [end_points] rack
    id => INT32
    end_points => port host security_protocol_type
      port => INT32
      host => STRING
      security_protocol_type => INT16
    rack => NULLABLE_STRING

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
partition_states	null
topic	Name of topic
partition	Topic partition id
controller_epoch	The controller epoch.
leader	The broker id for the leader.
leader_epoch	The leader epoch.
isr	The in sync replica ids.
zk_version	The ZK version.
replicas	The replica ids.
live_brokers	null
id	The broker id.
end_points	null
port	The port on which the broker accepts requests.
host	The hostname of the broker.
security_protocol_type	The security protocol type.
rack	The rack");

kafka_message!("UpdateMetadata Request (Version: 3) => controller_id controller_epoch [partition_states] [live_brokers]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas]
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
  live_brokers => id [end_points] rack
    id => INT32
    end_points => port host listener_name security_protocol_type
      port => INT32
      host => STRING
      listener_name => STRING
      security_protocol_type => INT16
    rack => NULLABLE_STRING

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
partition_states	null
topic	Name of topic
partition	Topic partition id
controller_epoch	The controller epoch.
leader	The broker id for the leader.
leader_epoch	The leader epoch.
isr	The in sync replica ids.
zk_version	The ZK version.
replicas	The replica ids.
live_brokers	null
id	The broker id.
end_points	null
port	The port on which the broker accepts requests.
host	The hostname of the broker.
listener_name	The listener name.
security_protocol_type	The security protocol type.
rack	The rack");

kafka_message!("UpdateMetadata Request (Version: 4) => controller_id controller_epoch [partition_states] [live_brokers]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas] [offline_replicas]
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
    offline_replicas => INT32
  live_brokers => id [end_points] rack
    id => INT32
    end_points => port host listener_name security_protocol_type
      port => INT32
      host => STRING
      listener_name => STRING
      security_protocol_type => INT16
    rack => NULLABLE_STRING

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
partition_states	null
topic	Name of topic
partition	Topic partition id
controller_epoch	The controller epoch.
leader	The broker id for the leader.
leader_epoch	The leader epoch.
isr	The in sync replica ids.
zk_version	The ZK version.
replicas	The replica ids.
offline_replicas	The offline replica ids
live_brokers	null
id	The broker id.
end_points	null
port	The port on which the broker accepts requests.
host	The hostname of the broker.
listener_name	The listener name.
security_protocol_type	The security protocol type.
rack	The rack");

kafka_message!("UpdateMetadata Response (Version: 0) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code");

kafka_message!("UpdateMetadata Response (Version: 1) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code");

kafka_message!("UpdateMetadata Response (Version: 2) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code");

kafka_message!("UpdateMetadata Response (Version: 3) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code");

kafka_message!("UpdateMetadata Response (Version: 4) => error_code
  error_code => INT16

Field 	Description
error_code	Response error code");
