kafka_message!("WriteTxnMarkers Request (Version: 0) => [transaction_markers]
  transaction_markers => producer_id producer_epoch transaction_result [topics] coordinator_epoch
    producer_id => INT64
    producer_epoch => INT16
    transaction_result => BOOLEAN
    topics => topic [partitions]
      topic => STRING
      partitions => INT32
    coordinator_epoch => INT32

Field 	Description
transaction_markers	The transaction markers to be written.
producer_id	Current producer id in use by the transactional id.
producer_epoch	Current epoch associated with the producer id.
transaction_result	The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
topics	The partitions to write markers for.
topic	Name of topic
partitions	null
coordinator_epoch	Epoch associated with the transaction state partition hosted by this transaction coordinator");

kafka_message!("WriteTxnMarkers Response (Version: 0) => [transaction_markers]
  transaction_markers => producer_id [topics]
    producer_id => INT64
    topics => topic [partitions]
      topic => STRING
      partitions => partition error_code
        partition => INT32
        error_code => INT16

Field 	Description
transaction_markers	Errors per partition from writing markers.
producer_id	Current producer id in use by the transactional id.
topics	Errors per partition from writing markers.
topic	Name of topic
partitions	null
partition	Topic partition id
error_code	Response error code");
