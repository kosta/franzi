//! Note: Currently, only Fetch Version 6 is implemented, implementing Version 7+ is blocked
//! by allowing different fields with the same name in `kafka_message!` (TODO).

// TODO: OMFG what a hack!
use crate as franzi_proto;

kafka_message!("Fetch Request (Version: 6) => replica_id max_wait_time min_bytes max_bytes isolation_level [topics]
  replica_id => INT32
  max_wait_time => INT32
  min_bytes => INT32
  max_bytes => INT32
  isolation_level => INT8
  topics => topic [partitions]
    topic => STRING
    partitions => partition fetch_offset log_start_offset partition_max_bytes
      partition => INT32
      fetch_offset => INT64
      log_start_offset => INT64
      partition_max_bytes => INT32

Field 	Description
replica_id	Broker id of the follower. For normal consumers, use -1.
max_wait_time	Maximum time in ms to wait for the response.
min_bytes	Minimum bytes to accumulate in the response.
max_bytes	Maximum bytes to accumulate in the response. Note that this is not an absolute maximum, if the first message in the first non-empty partition of the fetch is larger than this value, the message will still be returned to ensure that progress can be made.
isolation_level	This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
topics	Topics to fetch in the order provided.
topic	Name of topic
partitions	Partitions to fetch.
partition	Topic partition id
fetch_offset	Message offset.
log_start_offset	Earliest available offset of the follower replica. The field is only used when request is sent by follower.
partition_max_bytes	Maximum bytes to fetch.");

kafka_message!("Fetch Response (Version: 6) => throttle_time_ms [responses]
  throttle_time_ms => INT32
  responses => topic [partition_responses]
    topic => STRING
    partition_responses => partition_header record_set
      partition_header => partition error_code high_watermark last_stable_offset log_start_offset [aborted_transactions]
        partition => INT32
        error_code => INT16
        high_watermark => INT64
        last_stable_offset => INT64
        log_start_offset => INT64
        aborted_transactions => producer_id first_offset
          producer_id => INT64
          first_offset => INT64
      record_set => RECORDS

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
responses	null
topic	Name of topic
partition_responses	null
partition_header	null
partition	Topic partition id
error_code	Response error code
high_watermark	Last committed offset.
last_stable_offset	The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
log_start_offset	Earliest available offset.
aborted_transactions	null
producer_id	The producer id associated with the aborted transactions
first_offset	The first offset in the aborted transaction
record_set	null");