use bytes::{BufMut, Bytes};
use franzi_base::{
    read_vari64,
    types::{vi64, VarintBytes},
};
use franzi_base::{FromBytesError, FromKafkaBytes, ToKafkaBytes};
use std::io::Cursor;

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
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Record {
    pub length: vi64,
    ///bit 0~7: unused
    pub attributes: i8,
    pub timestamp_delta: vi64,
    pub offset_delta: vi64,
    pub key: VarintBytes,
    pub value: VarintBytes,
    pub headers: Vec<RecordHeader>,
}

impl FromKafkaBytes for Record {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let length = vi64::read(bytes)?;
        let attributes = i8::read(bytes)?;
        let timestamp_delta = vi64::read(bytes)?;
        let offset_delta = vi64::read(bytes)?;
        let key = VarintBytes::read(bytes)?;
        let value = VarintBytes::read(bytes)?;
        let headers = read_varint_vec(bytes)?;
        Ok(Record {
            length,
            attributes,
            timestamp_delta,
            offset_delta,
            key,
            value,
            headers,
        })
    }
}

impl ToKafkaBytes for Record {
    fn len_to_write(&self) -> usize {
        unimplemented!("TODO: Record::len_to_write")
    }

    fn write(&self, _bytes: &mut dyn BufMut) {
        unimplemented!("TODO: Record::write")
    }
}

fn read_varint_vec<T: FromKafkaBytes + std::fmt::Debug>(
    bytes: &mut Cursor<Bytes>,
) -> Result<Vec<T>, FromBytesError> {
    let item_len: i64 = read_vari64(bytes)?.into();
    if item_len < 0 {
        return Ok(Vec::new());
    }
    let item_len = item_len as usize;
    let mut vec = Vec::with_capacity(item_len);
    for _ in 0..item_len {
        vec.push(T::read(bytes)?);
    }
    Ok(vec)
}

/// ```ignore
/// headerKeyLength: varint
/// headerKey: String
/// headerValueLength: varint
/// Value: byte[]
/// ```
#[derive(FromKafkaBytes, ToKafkaBytes, Debug, PartialEq, Eq)]
pub struct RecordHeader {
    pub key: VarintBytes,
    pub value: VarintBytes,
}

// TODO: Implement old MessageSets
// see http://kafka.apache.org/documentation/#messageset

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use franzi_base::FromKafkaBytes;
    use lazy_static::lazy_static;
    use std::io::Cursor;

    struct Test {
        name: &'static str,
        expected: Records,
        encoded: &'static [u8],
    }

    lazy_static! {
        static ref TESTS: [Test; 3] = [
        Test {
            name: "empty message",
            expected: Records {
                batch_length: 49,
                magic: 2,
                crc: 1499445213,
                records: Some(Vec::new()),
                ..Default::default()
            },
            encoded: &[
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
            ],
        },
        Test {
            // TODO: assert!(test.attributes.is_control_batch())
            name: "control batch",
            expected: Records {
                batch_length: 49,
                magic: 2,
                crc: 1361986521,
                attributes: 32,
                records: Some(Vec::new()),
                ..Default::default()
            },
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 49, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2,               // Version
                81, 46, 67, 217, // CRC
                0, 32, // Attributes
                0, 0, 0, 0, // Last Offset Delta
                0, 0, 0, 0, 0, 0, 0, 0, // First Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
                0, 0, // Producer Epoch
                0, 0, 0, 0, // First Sequence
                0, 0, 0, 0, // Number of Records
            ]
        },
        Test {
            name: "uncompressed record",
            expected: Records{
                batch_length: 70,
                magic: 2,
                crc: 1417241085,
                first_timestamp: 1479847795000,
                max_timestamp: 0,
                last_offset_delta: 0,
                records: Some(vec![
                    Record{
                        length: vi64(20),
                        timestamp_delta: 5.into(),
                        key: Bytes::from(&[1, 2, 3, 4][..]).into(),
                        value: Bytes::from(&[5, 6, 7][..]).into(),
                        headers: vec![
                            RecordHeader{
                                key: Bytes::from(&[8, 9, 10][..]).into(),
                                value: Bytes::from(&[11, 12][..]).into(),
                            }
                        ],
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            },
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 70, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2,                // Version
                84, 121, 97, 253, // CRC
                0, 0, // Attributes
                0, 0, 0, 0, // Last Offset Delta
                0, 0, 1, 88, 141, 205, 89, 56, // First Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
                0, 0, // Producer Epoch
                0, 0, 0, 0, // First Sequence
                0, 0, 0, 1, // Number of Records
                40, // Record Length
                0,  // Attributes
                10, // Timestamp Delta
                0,  // Offset Delta
                8,  // Key Length
                1, 2, 3, 4,
                6, // Value Length
                5, 6, 7,
                2,        // Number of Headers
                6,        // Header Key Length
                8, 9, 10, // Header Key
                4,      // Header Value Length
                11, 12, // Header Value
            ],
        },
        ];
    }

    /// byte pattern stolen from sarama
    #[test]
    pub fn records_from_bytes() {
        for test in &*TESTS {
            assert_eq!(
                test.expected,
                Records::read(&mut Cursor::new(test.encoded.into())).unwrap(),
                "{}",
                test.name
            );
        }
    }
}
