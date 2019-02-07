kafka_message!(
    "ControlledShutdown Request (Version: 0) => broker_id
  broker_id => INT32

Field 	Description
broker_id	The id of the broker for which controlled shutdown has been requested."
);

kafka_message!(
    "ControlledShutdown Request (Version: 1) => broker_id
  broker_id => INT32

Field 	Description
broker_id	The id of the broker for which controlled shutdown has been requested."
);

kafka_message!(
    "ControlledShutdown Response (Version: 0) => error_code [partitions_remaining]
  error_code => INT16
  partitions_remaining => topic partition
    topic => STRING
    partition => INT32

Field 	Description
error_code	Response error code
partitions_remaining	The partitions that the broker still leads.
topic	Name of topic
partition	Topic partition id"
);

kafka_message!(
    "ControlledShutdown Response (Version: 1) => error_code [partitions_remaining]
  error_code => INT16
  partitions_remaining => topic partition
    topic => STRING
    partition => INT32

Field 	Description
error_code	Response error code
partitions_remaining	The partitions that the broker still leads.
topic	Name of topic
partition	Topic partition id"
);
