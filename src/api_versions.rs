use std::io::Cursor;
use bytes::{Bytes, BufMut};
use crate::{FromBytes, FromBytesError, ToBytes};

// TODO: Versioning concept?!

#[derive(Debug)]
pub struct ApiVersionsRequest2;

// TODO: Derive these!
impl FromBytes for ApiVersionsRequest2 {
    fn read(_bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ApiVersionsRequest2)
    }
}

impl ToBytes for ApiVersionsRequest2 {
    fn len_to_write(&self) -> usize {
        0
    }

    fn write(&self, _bytes: &mut BufMut) {
    }
}

///ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
#[derive(Debug)]
pub struct ApiVersionsResponse2 {
    /// Response error code
    pub error_code: i16,
    /// API versions supported by the broker.
    pub api_versions: Vec<ApiVersions2>,
    /// Duration in milliseconds for which the request was throttled due to quota violation (Zero if the request did not violate any quota)
    pub throttle_time_ms: i32,
}

#[derive(Debug)]
pub struct ApiVersions2 {
    /// API key
    pub api_key: i16,
    /// Minimum supported version.
    pub min_version: i16,
    /// Maximum supported version.
    pub max_version: i16,
}

impl FromBytes for ApiVersionsResponse2 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ApiVersionsResponse2{
            error_code: FromBytes::read(bytes)?,
            api_versions: FromBytes::read(bytes)?,
            throttle_time_ms: FromBytes::read(bytes)?,
        })
    }
}

impl FromBytes for ApiVersions2 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(ApiVersions2{
            api_key: FromBytes::read(bytes)?,
            min_version: FromBytes::read(bytes)?,
            max_version: FromBytes::read(bytes)?,
        })
    }
}

impl ToBytes for ApiVersionsResponse2 {
    fn len_to_write(&self) -> usize {
        self.error_code.len_to_write() +
        self.api_versions.len_to_write() +
        self.throttle_time_ms.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.error_code.write(bytes);
        self.api_versions.write(bytes);
        self.throttle_time_ms.write(bytes);
    }
}

impl ToBytes for ApiVersions2 {
    fn len_to_write(&self) -> usize {
        self.api_key.len_to_write() +
        self.min_version.len_to_write() +
        self.max_version.len_to_write()
    }

    fn write(&self, bytes: &mut BufMut) {
        self.api_key.write(bytes);
        self.min_version.write(bytes);
        self.max_version.write(bytes);
    }
}


