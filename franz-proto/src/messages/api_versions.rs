use franz_macros::kafka_message;

// #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
// pub struct ApiVersionsRequest2;

kafka_message!("ApiVersions Request (Version: 2) => ");

///ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ApiVersionsResponse2 {
    /// Response error code
    pub error_code: i16,
    /// API versions supported by the broker.
    pub api_versions: Option<Vec<ApiVersionsResponse2_Versions>>,
    /// Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
    pub throttle_time_ms: i32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
pub struct ApiVersionsResponse2_Versions {
    /// API key
    pub api_key: i16,
    /// Minimum supported version.
    pub min_version: i16,
    /// Maximum supported version.
    pub max_version: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::write_then_read_eq;

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsRequest2() {
        write_then_read_eq(ApiVersionsRequestV2 {}, b"");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsResponse2() {
        write_then_read_eq(
            ApiVersionsResponse2 {
                error_code: 123,
                api_versions: Some(vec![
                    ApiVersionsResponse2_Versions {
                        api_key: 4,
                        min_version: 0,
                        max_version: 9,
                    },
                    ApiVersionsResponse2_Versions {
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
