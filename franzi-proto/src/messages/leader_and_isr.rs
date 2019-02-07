kafka_message!("LeaderAndIsr Request (Version: 0) => controller_id controller_epoch [partition_states] [live_leaders]
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
  live_leaders => id host port
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
live_leaders	null
id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.");

kafka_message!("LeaderAndIsr Request (Version: 1) => controller_id controller_epoch [partition_states] [live_leaders]
  controller_id => INT32
  controller_epoch => INT32
  partition_states => topic partition controller_epoch leader leader_epoch [isr] zk_version [replicas] is_new
    topic => STRING
    partition => INT32
    controller_epoch => INT32
    leader => INT32
    leader_epoch => INT32
    isr => INT32
    zk_version => INT32
    replicas => INT32
    is_new => BOOLEAN
  live_leaders => id host port
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
is_new	Whether the replica should have existed on the broker or not
live_leaders	null
id	The broker id.
host	The hostname of the broker.
port	The port on which the broker accepts requests.");

kafka_message!(
    "LeaderAndIsr Response (Version: 0) => error_code [partitions]
  error_code => INT16
  partitions => topic partition error_code
    topic => STRING
    partition => INT32
    error_code => INT16

Field 	Description
error_code	Response error code
partitions	null
topic	Name of topic
partition	Topic partition id
error_code	Response error code"
);

kafka_message!(
    "LeaderAndIsr Response (Version: 1) => error_code [partitions]
  error_code => INT16
  partitions => topic partition error_code
    topic => STRING
    partition => INT32
    error_code => INT16

Field 	Description
error_code	Response error code
partitions	null
topic	Name of topic
partition	Topic partition id
error_code	Response error code"
);
