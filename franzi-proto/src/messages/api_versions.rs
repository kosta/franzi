kafka_message!("ApiVersions Request (Version: 0) =>

Field 	Description");

kafka_message!("ApiVersions Request (Version: 1) =>

Field 	Description");

kafka_message!("ApiVersions Request (Version: 2) =>

Field 	Description");

kafka_message!("ApiVersions Response (Version: 0) => error_code [api_versions]
  error_code => INT16
  api_versions => api_key min_version max_version
    api_key => INT16
    min_version => INT16
    max_version => INT16

Field 	Description
error_code	Response error code
api_versions	API versions supported by the broker.
api_key	API key.
min_version	Minimum supported version.
max_version	Maximum supported version.");

kafka_message!("ApiVersions Response (Version: 1) => error_code [api_versions] throttle_time_ms
  error_code => INT16
  api_versions => api_key min_version max_version
    api_key => INT16
    min_version => INT16
    max_version => INT16
  throttle_time_ms => INT32

Field 	Description
error_code	Response error code
api_versions	API versions supported by the broker.
api_key	API key.
min_version	Minimum supported version.
max_version	Maximum supported version.
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)");

kafka_message!("ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
  error_code => INT16
  api_versions => api_key min_version max_version
    api_key => INT16
    min_version => INT16
    max_version => INT16
  throttle_time_ms => INT32

Field 	Description
error_code	Response error code
api_versions	API versions supported by the broker.
api_key	API key.
min_version	Minimum supported version.
max_version	Maximum supported version.
throttle_time_ms	Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::write_then_read_eq;

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsRequestV2() {
        write_then_read_eq(ApiVersionsRequestV2 {}, b"");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsResponseV2() {
        write_then_read_eq(
            ApiVersionsResponseV2 {
                error_code: 123,
                api_versions: Some(vec![
                    ApiVersionsResponseV2_api_versions {
                        api_key: 4,
                        min_version: 0,
                        max_version: 9,
                    },
                    ApiVersionsResponseV2_api_versions {
                        api_key: 9,
                        min_version: -5,
                        max_version: 3,
                    },
                ]),
                throttle_time_ms: 1073741831,
            },
            b"\0{\0\0\0\x02\0\x04\0\0\0\t\0\t\xff\xfb\0\x03@\0\0\x07",
        );
    }
}
