kafka_message!("StopReplica Request (Version: 0) => controller_id controller_epoch delete_partitions [partitions]
  controller_id => INT32
  controller_epoch => INT32
  delete_partitions => BOOLEAN
  partitions => topic partition
    topic => STRING
    partition => INT32

Field 	Description
controller_id	The controller id.
controller_epoch	The controller epoch.
delete_partitions	Boolean which indicates if replica's partitions must
be deleted.
partitions	null
topic	Name of topic
partition	Topic partition id");

kafka_message!("StopReplica Response (Version: 0) => error_code [partitions]
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
error_code	Response error code");
