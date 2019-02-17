// Code adapted from https://github.com/tafia/quick-protobuf/blob/4a7a2f19ba0b12fadd0744f5cdf45e60a30259a0/quick-protobuf/src/reader.rs
// and https://github.com/tafia/quick-protobuf/blob/4a7a2f19ba0b12fadd0744f5cdf45e60a30259a0/quick-protobuf/src/sizeofs.rs
// TODO: Check with @tafia (Johann Tuffe) regarding license

use std::io::Cursor;
use std::mem::size_of;

use bytes::{Buf, BufMut, Bytes};

use crate::FromBytesError;

fn read_u8(bytes: &mut Cursor<Bytes>) -> Result<u8, FromBytesError> {
    if bytes.remaining() < size_of::<u8>() {
        return Err(FromBytesError);
    }
    Ok(bytes.get_u8())
}

/// Reads the next varint encoded u64
#[inline(always)]
pub fn read_varint32(bytes: &mut Cursor<Bytes>) -> Result<u32, FromBytesError> {
    let mut b = read_u8(bytes)?;
    if b & 0x80 == 0 {
        return Ok(b as u32);
    }
    let mut r = (b & 0x7f) as u32;

    b = read_u8(bytes)?;
    r |= ((b & 0x7f) as u32) << 7;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= ((b & 0x7f) as u32) << 14;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= ((b & 0x7f) as u32) << 21;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= ((b & 0xf) as u32) << 28;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    // discards extra bytes
    for _ in 0..5 {
        if read_u8(bytes)? & 0x80 == 0 {
            return Ok(r);
        }
    }

    // cannot read more than 10 bytes
    Err(FromBytesError)
}

/// Reads the next varint encoded u64
#[inline(always)]
pub fn read_varint64(bytes: &mut Cursor<Bytes>) -> Result<u64, FromBytesError> {
    // part0
    let mut b = read_u8(bytes)?;
    if b & 0x80 == 0 {
        return Ok(b as u64);
    }
    let mut r0 = (b & 0x7f) as u32;

    b = read_u8(bytes)?;
    r0 |= ((b & 0x7f) as u32) << 7;
    if b & 0x80 == 0 {
        return Ok(r0 as u64);
    }

    b = read_u8(bytes)?;
    r0 |= ((b & 0x7f) as u32) << 14;
    if b & 0x80 == 0 {
        return Ok(r0 as u64);
    }

    b = read_u8(bytes)?;
    r0 |= ((b & 0x7f) as u32) << 21;
    if b & 0x80 == 0 {
        return Ok(r0 as u64);
    }

    // part1
    b = read_u8(bytes)?;
    let mut r1 = (b & 0x7f) as u32;
    if b & 0x80 == 0 {
        return Ok(r0 as u64 | (r1 as u64) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= ((b & 0x7f) as u32) << 7;
    if b & 0x80 == 0 {
        return Ok(r0 as u64 | (r1 as u64) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= ((b & 0x7f) as u32) << 14;
    if b & 0x80 == 0 {
        return Ok(r0 as u64 | (r1 as u64) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= ((b & 0x7f) as u32) << 21;
    if b & 0x80 == 0 {
        return Ok(r0 as u64 | (r1 as u64) << 28);
    }

    // part2
    b = read_u8(bytes)?;
    let mut r2 = (b & 0x7f) as u32;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56);
    }

    b = read_u8(bytes)?;
    r2 |= (b as u32) << 7;
    if b & 0x80 == 0 {
        return Ok((r0 as u64 | (r1 as u64) << 28) | (r2 as u64) << 56);
    }

    // cannot read more than 10 bytes
    Err(FromBytesError)
}

#[inline(always)]
pub fn sizeof_varint(v: u64) -> usize {
    match v {
        0x0...0x7F => 1,
        0x80...0x3FFF => 2,
        0x4000...0x1FFFFF => 3,
        0x200000...0xFFFFFFF => 4,
        0x10000000...0x7FFFFFFFF => 5,
        0x0800000000...0x3FFFFFFFFFF => 6,
        0x040000000000...0x1FFFFFFFFFFFF => 7,
        0x02000000000000...0xFFFFFFFFFFFFFF => 8,
        0x0100000000000000...0x7FFFFFFFFFFFFFFF => 9,
        _ => 10,
    }
}

#[inline(always)]
pub fn write_varint(bytes: &mut BufMut, mut v: u64) {
    while v > 0x7F {
        bytes.put_u8(((v as u8) & 0x7F) | 0x80);
        v >>= 7;
    }
    bytes.put_u8(v as u8)
}
