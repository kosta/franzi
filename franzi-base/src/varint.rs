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
        return Ok(u32::from(b));
    }
    let mut r = u32::from(b & 0x7f);

    b = read_u8(bytes)?;
    r |= (u32::from(b & 0x7f)) << 7;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= (u32::from(b & 0x7f)) << 14;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= (u32::from(b & 0x7f)) << 21;
    if b & 0x80 == 0 {
        return Ok(r);
    }

    b = read_u8(bytes)?;
    r |= (u32::from(b & 0xf)) << 28;
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
        return Ok(u64::from(b));
    }
    let mut r0 = u32::from(b & 0x7f);

    b = read_u8(bytes)?;
    r0 |= (u32::from(b & 0x7f)) << 7;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0));
    }

    b = read_u8(bytes)?;
    r0 |= (u32::from(b & 0x7f)) << 14;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0));
    }

    b = read_u8(bytes)?;
    r0 |= (u32::from(b & 0x7f)) << 21;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0));
    }

    // part1
    b = read_u8(bytes)?;
    let mut r1 = u32::from(b & 0x7f);
    if b & 0x80 == 0 {
        return Ok(u64::from(r0) | u64::from(r1) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= (u32::from(b & 0x7f)) << 7;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0) | u64::from(r1) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= (u32::from(b & 0x7f)) << 14;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0) | u64::from(r1) << 28);
    }

    b = read_u8(bytes)?;
    r1 |= (u32::from(b & 0x7f)) << 21;
    if b & 0x80 == 0 {
        return Ok(u64::from(r0) | u64::from(r1) << 28);
    }

    // part2
    b = read_u8(bytes)?;
    let mut r2 = u32::from(b & 0x7f);
    if b & 0x80 == 0 {
        return Ok((u64::from(r0) | u64::from(r1) << 28) | u64::from(r2) << 56);
    }

    b = read_u8(bytes)?;
    r2 |= (u32::from(b)) << 7;
    if b & 0x80 == 0 {
        return Ok((u64::from(r0) | u64::from(r1) << 28) | u64::from(r2) << 56);
    }

    // cannot read more than 10 bytes
    Err(FromBytesError)
}

#[inline(always)]
pub fn sizeof_varint(v: u64) -> usize {
    match v {
        0x0...0x7F => 1,
        0x80...0x3FFF => 2,
        0x4000...0x1F_FFFF => 3,
        0x20_0000...0xFFF_FFFF => 4,
        0x1000_0000...0x007_FFFF_FFFF => 5,
        0x08_0000_0000...0x03FF_FFFF_FFFF => 6,
        0x0400_0000_0000...0x0001_FFFF_FFFF_FFFF => 7,
        0x0002_0000_0000_0000...0x00FF_FFFF_FFFF_FFFF => 8,
        0x0100_0000_0000_0000...0x7FFF_FFFF_FFFF_FFFF => 9,
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
