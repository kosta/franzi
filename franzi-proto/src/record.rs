use franzi_base::types::{vi64, VarintBytes};

/// RECORDS	Represents a sequence of Kafka records as NULLABLE_BYTES.
/// [Kafka Spec](http://kafka.apache.org/documentation/#recordbatch):
/// ```ignore
/// baseOffset: int64
/// batchLength: int32
/// partitionLeaderEpoch: int32
/// magic: int8 (current magic value is 2)
/// crc: int32
/// attributes: int16
///     bit 0~2:
///         0: no compression
///         1: gzip
///         2: snappy
///         3: lz4
///         4: zstd
///     bit 3: timestampType
///     bit 4: isTransactional (0 means not transactional)
///     bit 5: isControlBatch (0 means not a control batch)
///     bit 6~15: unused
/// lastOffsetDelta: int32
/// firstTimestamp: int64
/// maxTimestamp: int64
/// producerId: int64
/// producerEpoch: int16
/// baseSequence: int32
/// records: [Record]
/// ```
///
/// See also http://kafka.apache.org/documentation/#upgrade_11_message_format
/// and https://cwiki.apache.org/confluence/display/KAFKA/KIP-98+-+Exactly+Once+Delivery+and+Transactional+Messaging#KIP-98-ExactlyOnceDeliveryandTransactionalMessaging-MessageFormat
#[derive(FromKafkaBytes, ToKafkaBytes, Default, Debug, PartialEq, Eq)]
pub struct Records {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    /// current magic value is 2
    pub magic: i8,
    pub crc: i32,
    /// bit 0~2:
    ///     0: no compression
    ///     1: gzip
    ///     2: snappy
    ///     3: lz4
    ///     4: zstd
    /// bit 3: timestampType
    /// bit 4: isTransactional (0 means not transactional)
    /// bit 5: isControlBatch (0 means not a control batch)
    /// bit 6~15: unused
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: Option<Vec<Record>>,
}

/// RECORDS	Represents a sequence of Kafka records as NULLABLE_BYTES.
/// [Kafka Spec](http://kafka.apache.org/documentation/#recordbatch):
/// ```ignore
/// length: varint
/// attributes: int8
///     bit 0~7: unused
/// timestampDelta: varint
/// offsetDelta: varint
/// keyLength: varint
/// key: byte[]
/// valueLen: varint
/// value: byte[]
/// Headers => [Header]
/// ```
#[derive(FromKafkaBytes, ToKafkaBytes, Debug, PartialEq, Eq)]
pub struct Record {
    pub length: vi64,
    ///bit 0~7: unused
    pub attributes: i8,
    pub timestamp_delta: vi64,
    pub offset_delta: vi64,
    pub key: VarintBytes,
    pub value_len: VarintBytes,
    pub headers: Option<Vec<RecordHeader>>,
}

/// ```ignore
/// headerKeyLength: varint
/// headerKey: String
/// headerValueLength: varint
/// Value: byte[]
/// ```
#[derive(FromKafkaBytes, ToKafkaBytes, Debug, PartialEq, Eq)]
pub struct RecordHeader {
    pub header_key: VarintBytes,
    pub value: VarintBytes,
}

// TODO: Implement old MessageSets
// see http://kafka.apache.org/documentation/#messageset

#[cfg(test)]
mod tests {
    use franzi_base::FromKafkaBytes;
    use std::io::Cursor;
    use super::*;

    /// byte pattern stolen from sarama
    #[test]
    pub fn empty_message() {
        let expected = Records {
            batch_length: 49,
            magic: 2,
            crc: 1499445213,
            records: Some(Vec::new()),
            ..Default::default()
        };
        let encoded: &[u8] = &[
            0, 0, 0, 0, 0, 0, 0, 0, // First Offset
            0, 0, 0, 49, // Length
            0, 0, 0, 0, // Partition Leader Epoch
            2, // Version
            89, 95, 183, 221, // CRC
            0, 0, // Attributes
            0, 0, 0, 0, // Last Offset Delta
            0, 0, 0, 0, 0, 0, 0, 0, // First Timestamp
            0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
            0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
            0, 0, // Producer Epoch
            0, 0, 0, 0, // First Sequence
            0, 0, 0, 0, // Number of Records
        ];
        assert_eq!(expected, Records::read(&mut Cursor::new(encoded.into())).unwrap());
    }
}
