// OffsetFetch Request (Version: 5) => group_id [topics]
//   group_id => STRING
//   topics => topic [partitions]
//     topic => STRING
//     partitions => partition
//       partition => INT32

// OffsetFetch Response (Version: 5) => throttle_time_ms [responses] error_code
//   throttle_time_ms => INT32
//   responses => topic [partition_responses]
//     topic => STRING
//     partition_responses => partition offset leader_epoch metadata error_code
//       partition => INT32
//       offset => INT64
//       leader_epoch => INT32
//       metadata => NULLABLE_STRING
//       error_code => INT16
//   error_code => INT16

// throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
// responses	Responses by topic for fetched offsets
// topic	Name of topic
// partition_responses	Responses by partition for fetched offsets
// partition	Topic partition id
// offset	Message offset to be committed
// leader_epoch	The leader epoch, if provided is derived from the last consumed record. This is used by the consumer to check for log truncation and to ensure partition metadata is up to date following a group rebalance.
// metadata	Any associated metadata the client wants to keep.
// error_code	Response error code
