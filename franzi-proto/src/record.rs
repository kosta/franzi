use bytes::{BufMut, Bytes};
use franzi_base::{
    read_vari64,
    types::{vi64, VarintBytes},
};
use franzi_base::{FromBytesError, FromKafkaBytes, ToKafkaBytes};
use std::{convert::TryFrom, io::Cursor};
use tracing::{event, Level};

#[derive(Debug, PartialEq, Eq)]
pub struct Records(pub Vec<Record>);

impl ToKafkaBytes for Records {
    fn len_to_write(&self) -> usize {
        unimplemented!("TODO: Records::len_to_write")
    }

    fn write(&self, _bytes: &mut dyn BufMut) {
        unimplemented!("TODO: Records::write")
    }
}

impl FromKafkaBytes for Records {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let item_len = i32::read(bytes)?;
        if item_len < 0 {
            return Ok(Records(Vec::new()));
        }
        let item_len = item_len as usize;
        let mut vec = Vec::with_capacity(item_len);
        let buffer_len = bytes.get_ref().len() as u64;
        for _ in 0..item_len {
            if bytes.position() >= buffer_len {
                // incomplete records are ok, it seems?
                event!(
                    Level::DEBUG,
                    ?buffer_len,
                    "Records::read: breaking at end of buffer"
                );
                break;
            }
            match Record::read(bytes) {
                Ok(record) => vec.push(record),
                Err(FromBytesError::UnexpectedEOF) => {
                    // incomplete record, again
                    event!(Level::DEBUG, "Records::read: breaking at incomplete record");
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(Records(vec))
    }
}

// pub type Records = RecordEnum;
#[derive(Debug, PartialEq, Eq)]
pub enum Record {
    // V0(MessageSetV0),
    // V1(MessageSetV1),
    V2(RecordsV2),
}

impl FromKafkaBytes for Record {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let magic_byte = *bytes
            .get_ref()
            .get(bytes.position() as usize + 16)
            .ok_or(FromBytesError::UnexpectedEOF)? as i8;
        match magic_byte {
            // 0 => Ok(Record::V0(MessageSetV0::read(bytes)?)),
            0 => Err(FromBytesError::Unimplemented(
                "Record with magic_byte (version) 0",
            )),
            1 => Err(FromBytesError::Unimplemented(
                "Record with magic_byte (version) 1",
            )),
            2 => Ok(Record::V2(RecordsV2::read(bytes)?)),
            _ => Err(FromBytesError::UnknownMagicByte(magic_byte)),
        }
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
pub struct RecordsV2Head {
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
}

#[derive(Copy, Clone, Debug)]
pub enum Compression {
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

impl TryFrom<i16> for Compression {
    type Error = i8;

    fn try_from(attributes: i16) -> Result<Compression, i8> {
        use Compression::*;
        match attributes & 0b111 {
            0 => Ok(None),
            1 => Ok(Gzip),
            2 => Ok(Snappy),
            3 => Ok(Lz4),
            4 => Ok(Zstd),
            x => Err(x as i8),
        }
    }
}

impl Compression {
    pub fn decompress(
        &self,
        bytes: &mut Cursor<Bytes>,
        batch_length: i32,
    ) -> Result<Bytes, FromBytesError> {
        use franzi_base::DecompressionError;
        use std::io::Read;
        use Compression::*;
        let pos = bytes.position() as usize;
        const BATCH_HEADER_LENGTH: usize = 49;
        let end_pos = pos + batch_length as usize - BATCH_HEADER_LENGTH;
        bytes.set_position(end_pos as u64);
        match self {
            None => Ok(bytes.get_ref().slice(pos, end_pos)),
            Gzip => {
                use flate2::read::GzDecoder;
                let slice = bytes
                    .get_ref()
                    .get(pos..end_pos)
                    .ok_or(FromBytesError::UnexpectedEOF)?;
                let mut gz = GzDecoder::new(slice);
                let mut decompressed = Vec::new();
                gz.read_to_end(&mut decompressed)
                    .map_err(|e| FromBytesError::Decompression(DecompressionError::Gzip(e)))?;
                Ok(decompressed.into())
            }
            Snappy => {
                let slice = bytes
                    .get_ref()
                    .get(pos..end_pos)
                    .ok_or(FromBytesError::UnexpectedEOF)?;
                snap::Decoder::new()
                    .decompress_vec(slice)
                    .map(|vec| vec.into())
                    .map_err(|e| FromBytesError::Decompression(DecompressionError::Snappy(e)))
            }
            Lz4 => {
                let slice = bytes
                    .get_ref()
                    .get(pos..end_pos)
                    .ok_or(FromBytesError::UnexpectedEOF)?;
                let mut decompressed = Vec::new();
                lz4::Decoder::new(slice)
                    .and_then(|mut decoder| decoder.read_to_end(&mut decompressed))
                    .map_err(|e| FromBytesError::Decompression(DecompressionError::Lz4(e)))?;
                Ok(decompressed.into())
            }
            Zstd => {
                let slice = bytes
                    .get_ref()
                    .get(pos..end_pos)
                    .ok_or(FromBytesError::UnexpectedEOF)?;
                zstd::stream::decode_all(slice)
                    .map(|vec| vec.into())
                    .map_err(|e| FromBytesError::Decompression(DecompressionError::Zstd(e)))
            }
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct RecordsV2 {
    pub head: RecordsV2Head,
    pub records: Vec<RecordV2>,
}

impl FromKafkaBytes for RecordsV2 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        let head: RecordsV2Head = FromKafkaBytes::read(bytes)?;
        if head.magic != 2 {
            return Err(FromBytesError::UnknownMagicByte(head.magic));
        }
        let records_len: i32 = FromKafkaBytes::read(bytes)?;
        let mut records = Vec::with_capacity(usize::try_from(records_len).unwrap_or_default());
        if records_len > 0 {
            let compression = Compression::try_from(head.attributes)
                .map_err(|x| FromBytesError::UnknownCompression(x))?;
            let decompressed_bytes = compression.decompress(bytes, head.batch_length)?;
            let mut decompressed = Cursor::new(decompressed_bytes);

            for _ in 0..records_len {
                records.push(RecordV2::read(&mut decompressed)?);
            }
        }
        Ok(RecordsV2 { head, records })
    }
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
pub struct RecordV2 {
    pub length: vi64,
    ///bit 0~7: unused
    pub attributes: i8,
    pub timestamp_delta: vi64,
    pub offset_delta: vi64,
    pub key: VarintBytes,
    pub value: VarintBytes,
    pub headers: Vec<RecordV2Header>,
}

impl FromKafkaBytes for RecordV2 {
    fn read(bytes: &mut Cursor<Bytes>) -> Result<Self, FromBytesError> {
        Ok(RecordV2 {
            length: vi64::read(bytes)?,
            attributes: i8::read(bytes)?,
            timestamp_delta: vi64::read(bytes)?,
            offset_delta: vi64::read(bytes)?,
            key: VarintBytes::read(bytes)?,
            value: VarintBytes::read(bytes)?,
            headers: read_varint_vec(bytes)?,
        })
    }
}

impl ToKafkaBytes for RecordV2 {
    fn len_to_write(&self) -> usize {
        unimplemented!("TODO: RecordV2::len_to_write")
    }

    fn write(&self, _bytes: &mut dyn BufMut) {
        unimplemented!("TODO: RecordV2::write")
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
pub struct RecordV2Header {
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
        expected: Record,
        encoded: &'static [u8],
    }

    lazy_static! {
        static ref TESTS: [Test; 6] = [
        Test {
            name: "empty message",
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                    batch_length: 49,
                    magic: 2,
                    crc: 1499445213,
                    ..Default::default()
                },
                records: Vec::new(),
            }),
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
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                batch_length: 49,
                magic: 2,
                crc: 1361986521,
                attributes: 32,
                ..Default::default()
                },
                records: Vec::new(),
            }),
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 49, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2, // Version
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
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                batch_length: 70,
                magic: 2,
                crc: 1417241085,
                first_timestamp: 1479847795000,
                max_timestamp: 0,
                last_offset_delta: 0,
                ..Default::default()
                },
                records: vec![
                    RecordV2{
                        length: vi64(20),
                        timestamp_delta: 5.into(),
                        key: Bytes::from(&[1, 2, 3, 4][..]).into(),
                        value: Bytes::from(&[5, 6, 7][..]).into(),
                        headers: vec![
                            RecordV2Header{
                                key: Bytes::from(&[8, 9, 10][..]).into(),
                                value: Bytes::from(&[11, 12][..]).into(),
                            }
                        ],
                        ..Default::default()
                    }
                ],
            }),
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 70, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2, // Version
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
                0, // Attributes
                10, // Timestamp Delta
                0, // Offset Delta
                8, // Key Length
                1, 2, 3, 4,
                6, // Value Length
                5, 6, 7,
                2, // Number of Headers
                6, // Header Key Length
                8, 9, 10, // Header Key
                4, // Header Value Length
                11, 12, // Header Value
            ],
        },
        Test {
            name: "gzip compressed record",
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                    batch_length: 94,
                    magic: 2,
                    crc: -1611876675,
                    attributes: 1,
                    first_timestamp: 1479847795000,
                    max_timestamp: 0,
                    last_offset_delta: 0,
                    ..Default::default()
                },
                records: vec![
                    RecordV2{
                        length: vi64(20),
                        timestamp_delta: 5.into(),
                        key: Bytes::from(&[1, 2, 3, 4][..]).into(),
                        value: Bytes::from(&[5, 6, 7][..]).into(),
                        headers: vec![
                            RecordV2Header{
                                key: Bytes::from(&[8, 9, 10][..]).into(),
                                value: Bytes::from(&[11, 12][..]).into(),
                            }
                        ],
                        ..Default::default()
                    }
                ],
            }),
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 94, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2, // Version
                159, 236, 182, 189, // CRC
                0, 1, // Attributes
                0, 0, 0, 0, // Last Offset Delta
                0, 0, 1, 88, 141, 205, 89, 56, // First Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
                0, 0, // Producer Epoch
                0, 0, 0, 0, // First Sequence
                0, 0, 0, 1, // Number of Records
                31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 210, 96, 224, 98, 224, 96, 100, 98, 102, 97, 99, 101,
                99, 103, 98, 227, 224, 228, 98, 225, 230, 1, 4, 0, 0, 255, 255, 173, 201, 88, 103, 21, 0, 0, 0,
            ],
        },
        Test {
            name: "snappy compressed record",
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                    batch_length: 72,
                    magic: 2,
                    crc: 352362337,
                    attributes: 2,
                    first_timestamp: 1479847795000,
                    max_timestamp: 0,
                    last_offset_delta: 0,
                    ..Default::default()
                },
                records: vec![
                    RecordV2{
                        length: vi64(20),
                        timestamp_delta: 5.into(),
                        key: Bytes::from(&[1, 2, 3, 4][..]).into(),
                        value: Bytes::from(&[5, 6, 7][..]).into(),
                        headers: vec![
                            RecordV2Header{
                                key: Bytes::from(&[8, 9, 10][..]).into(),
                                value: Bytes::from(&[11, 12][..]).into(),
                            }
                        ],
                        ..Default::default()
                    }
                ],
            }),
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 72, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2, // Version
                21, 0, 159, 97, // CRC
                0, 2, // Attributes
                0, 0, 0, 0, // Last Offset Delta
                0, 0, 1, 88, 141, 205, 89, 56, // First Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
                0, 0, // Producer Epoch
                0, 0, 0, 0, // First Sequence
                0, 0, 0, 1, // Number of Records
                21, 80, 40, 0, 10, 0, 8, 1, 2, 3, 4, 6, 5, 6, 7, 2, 6, 8, 9, 10, 4, 11, 12,
            ],
        },
        Test {
            name: "lz4 compressed record",
            expected: Record::V2(RecordsV2{
                head: RecordsV2Head {
                    batch_length: 89,
                    magic: 2,
                    crc: -1454737467,
                    attributes: 3,
                    first_timestamp: 1479847795000,
                    max_timestamp: 0,
                    last_offset_delta: 0,
                    ..Default::default()
                },
                records: vec![
                    RecordV2{
                        length: vi64(20),
                        timestamp_delta: 5.into(),
                        key: Bytes::from(&[1, 2, 3, 4][..]).into(),
                        value: Bytes::from(&[5, 6, 7][..]).into(),
                        headers: vec![
                            RecordV2Header{
                                key: Bytes::from(&[8, 9, 10][..]).into(),
                                value: Bytes::from(&[11, 12][..]).into(),
                            }
                        ],
                        ..Default::default()
                    }
                ],
            }),
            encoded: &[
                0, 0, 0, 0, 0, 0, 0, 0, // First Offset
                0, 0, 0, 89, // Length
                0, 0, 0, 0, // Partition Leader Epoch
                2, // Version
                169, 74, 119, 197, // CRC
                0, 3, // Attributes
                0, 0, 0, 0, // Last Offset Delta
                0, 0, 1, 88, 141, 205, 89, 56, // First Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Max Timestamp
                0, 0, 0, 0, 0, 0, 0, 0, // Producer ID
                0, 0, // Producer Epoch
                0, 0, 0, 0, // First Sequence
                0, 0, 0, 1, // Number of Records
                4, 34, 77, 24, 100, 112, 185, 21, 0, 0, 128, 40, 0, 10, 0, 8, 1, 2, 3, 4, 6, 5, 6, 7, 2,
                6, 8, 9, 10, 4, 11, 12, 0, 0, 0, 0, 12, 59, 239, 146,
            ],
        },
        // TODO: Test zstd compressed record
        ];
    }

    /// byte pattern stolen from sarama
    #[test]
    pub fn records_from_bytes() {
        for test in &*TESTS {
            assert_eq!(
                test.expected,
                Record::read(&mut Cursor::new(test.encoded.into())).unwrap(),
                "{}",
                test.name
            );
        }
    }
}
