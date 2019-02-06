kafka_message!("DeleteRecords Request (Version: 0) => [topics] timeout
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset
      partition => INT32
      offset => INT64
  timeout => INT32

Field 	Description
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
offset	The offset before which the messages will be deleted. -1 means
high-watermark for the partition.
timeout	The maximum time to await a response in ms.");

kafka_message!("DeleteRecords Request (Version: 1) => [topics] timeout
  topics => topic [partitions]
    topic => STRING
    partitions => partition offset
      partition => INT32
      offset => INT64
  timeout => INT32

Field 	Description
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
offset	The offset before which the messages will be deleted. -1 means
high-watermark for the partition.
timeout	The maximum time to await a response in ms.");

kafka_message!("DeleteRecords Response (Version: 0) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition low_watermark error_code
      partition => INT32
      low_watermark => INT64
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
low_watermark	Smallest available offset of all live replicas
error_code	Response error code");

kafka_message!("DeleteRecords Response (Version: 1) => throttle_time_ms [topics]
  throttle_time_ms => INT32
  topics => topic [partitions]
    topic => STRING
    partitions => partition low_watermark error_code
      partition => INT32
      low_watermark => INT64
      error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was
throttled due to quota violation (Zero if the request did not violate
any quota)
topics	null
topic	Name of topic
partitions	null
partition	Topic partition id
low_watermark	Smallest available offset of all live replicas
error_code	Response error code");
