kafka_message!("DeleteTopics Request (Version: 0) => [topics] timeout
  topics => STRING
  timeout => INT32

Field 	Description
topics	An array of topics to be deleted.
timeout	The time in ms to wait for a topic to be completely deleted on the controller node. Values <= 0 will trigger topic deletion and return immediately");

kafka_message!("DeleteTopics Request (Version: 1) => [topics] timeout
  topics => STRING
  timeout => INT32

Field 	Description
topics	An array of topics to be deleted.
timeout	The time in ms to wait for a topic to be completely deleted on the controller node. Values <= 0 will trigger topic deletion and return immediately");

kafka_message!("DeleteTopics Request (Version: 2) => [topics] timeout
  topics => STRING
  timeout => INT32

Field 	Description
topics	An array of topics to be deleted.
timeout	The time in ms to wait for a topic to be completely deleted on the controller node. Values <= 0 will trigger topic deletion and return immediately");

kafka_message!("DeleteTopics Request (Version: 3) => [topics] timeout
  topics => STRING
  timeout => INT32

Field 	Description
topics	An array of topics to be deleted.
timeout	The time in ms to wait for a topic to be completely deleted on the controller node. Values <= 0 will trigger topic deletion and return immediately");

kafka_message!("DeleteTopics Response (Version: 0) => [topic_error_codes]
  topic_error_codes => topic error_code
    topic => STRING
    error_code => INT16

Field 	Description
topic_error_codes	An array of per topic error codes.
topic	Name of topic
error_code	Response error code");

kafka_message!("DeleteTopics Response (Version: 1) => throttle_time_ms [topic_error_codes]
  throttle_time_ms => INT32
  topic_error_codes => topic error_code
    topic => STRING
    error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topic_error_codes	An array of per topic error codes.
topic	Name of topic
error_code	Response error code");

kafka_message!("DeleteTopics Response (Version: 2) => throttle_time_ms [topic_error_codes]
  throttle_time_ms => INT32
  topic_error_codes => topic error_code
    topic => STRING
    error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topic_error_codes	An array of per topic error codes.
topic	Name of topic
error_code	Response error code");

kafka_message!("DeleteTopics Response (Version: 3) => throttle_time_ms [topic_error_codes]
  throttle_time_ms => INT32
  topic_error_codes => topic error_code
    topic => STRING
    error_code => INT16

Field 	Description
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
topic_error_codes	An array of per topic error codes.
topic	Name of topic
error_code	Response error code");
