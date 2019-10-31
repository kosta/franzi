/// MessageSet (Version: 1) => [offset message_size message]
///     offset => INT64
///     message_size => INT32
///     message => crc magic_byte attributes key value
///         crc => INT32
///         magic_byte => INT8
///         attributes => INT8
///             bit 0~2:
///                 0: no compression
///                 1: gzip
///                 2: snappy
///                 3: lz4
///             bit 3: timestampType
///                 0: create time
///                 1: log append time
///             bit 4~7: unused
///         timestamp =>INT64
///         key => BYTES
///         value => BYTES

// TODO: Implement MessageSetV1
pub struct MessageSetV1;
