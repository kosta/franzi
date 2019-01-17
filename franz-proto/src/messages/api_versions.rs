use bytes::{BufMut, Bytes};
use franz_base::{FromBytes, FromBytesError, ToBytes};
use std::io::Cursor;

#[derive(Debug, Eq, PartialEq, FromBytes)]
pub struct ApiVersionsRequest2;

impl ToBytes for ApiVersionsRequest2 {
    fn len_to_write(&self) -> usize {
        0
    }

    fn write(&self, _bytes: &mut BufMut) {}
}

///ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
#[derive(Debug, Eq, PartialEq, FromBytes)]
pub struct ApiVersionsResponse2 {
    /// Response error code
    pub error_code: i16,
    /// API versions supported by the broker.
    pub api_versions: Option<Vec<ApiVersionsResponse2_Versions>>,
    /// Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
    pub throttle_time_ms: i32,
}

#[derive(Debug, Eq, PartialEq)]
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
    use crate::tests::write_then_read_eq;
    use super::*;
    use bytes::BytesMut;

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsRequest2() {
        write_then_read_eq(ApiVersionsRequest2{}, b"");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ApiVersionsResponse2() {
        write_then_read_eq(ApiVersionsResponse2{
            error_code: 123,
            api_versions: Some(vec!(ApiVersionsResponse2_Versions{
                api_key: 4,
                min_version: 0,
                max_version: 9,
            }, ApiVersionsResponse2_Versions{
                api_key: 9,
                min_version: -5,
                max_version: 3,
            })),
            throttle_time_ms: 1073741831,
        }, b"\0{\0\0\0\x02\0\x04\0\0\0\t\0\t\xff\xfb\0\x03@\0\0\x07");
    }
}

impl FromBytes for ApiVersionsResponse2_Versions {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ApiVersionsResponse2_Versions {
            api_key: FromBytes::read(bytes)?,
            min_version: FromBytes::read(bytes)?,
            max_version: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ApiVersionsResponse2 {
    fn len_to_write(&self) -> usize {
        self.error_code.len_to_write()
            + self.api_versions.len_to_write()
            + self.throttle_time_ms.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.error_code.write(bytes);
        self.api_versions.write(bytes);
        self.throttle_time_ms.write(bytes);
    }
}

impl ToBytes for ApiVersionsResponse2_Versions {
    fn len_to_write(&self) -> usize {
        self.api_key.len_to_write()
            + self.min_version.len_to_write()
            + self.max_version.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.api_key.write(bytes);
        self.min_version.write(bytes);
        self.max_version.write(bytes);
    }
}
