// Code adapted from https://github.com/tafia/quick-protobuf/blob/4a7a2f19ba0b12fadd0744f5cdf45e60a30259a0/quick-protobuf/src/reader.rs
// and https://github.com/tafia/quick-protobuf/blob/4a7a2f19ba0b12fadd0744f5cdf45e60a30259a0/quick-protobuf/src/sizeofs.rs
// TODO: Check with @tafia (Johann Tuffe) regarding license

#![allow(ellipsis_inclusive_range_patterns)]

use std::mem::size_of;

use bytes::{Buf, BufMut};

use crate::FromBytesError;

fn read_u8<B: Buf>(bytes: &mut B) -> Result<u8, FromBytesError> {
    if bytes.remaining() < size_of::<u8>() {
        return Err(FromBytesError::UnexpectedEOF);
    }
    Ok(bytes.get_u8())
}

/// Reads the next varint encoded u64
#[inline(always)]
pub fn read_varint32<B: Buf>(bytes: &mut B) -> Result<u32, FromBytesError> {
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
    Err(FromBytesError::VarIntOverflow)
}

/// Reads the next varint encoded u64
#[inline(always)]
pub fn read_varu64<B: Buf>(bytes: &mut B) -> Result<u64, FromBytesError> {
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
    Err(FromBytesError::VarIntOverflow)
}

pub fn read_vari64<B: Buf>(bytes: &mut B) -> Result<i64, FromBytesError> {
    let u = read_varu64(bytes)?;
    let x = (u as i64) >> 1;
    let negative = u & 1;
    if negative != 0 {
        Ok(x ^ -1)
    } else {
        Ok(x)
    }
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
pub fn write_varu64(bytes: &mut dyn BufMut, mut v: u64) {
    while v > 0x7F {
        bytes.put_u8(((v as u8) & 0x7F) | 0x80);
        v >>= 7;
    }
    bytes.put_u8(v as u8)
}

pub fn write_vari64(bytes: &mut dyn BufMut, v: i64) {
    let mut u = (v as u64) << 1;
    if v < 0 {
        u ^= u64::MAX;
    }
    write_varu64(bytes, u)
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};
    use quickcheck::quickcheck;

    const UNSIGNED: &[(u64, &[u8])] = &[
        (1, &[0x01]),
        (2, &[0x02]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (255, &[0xff, 0x01]),
        (256, &[0x80, 0x02]),
        (4223, &[0xff, 0x20]),
    ];

    const SIGNED: &[(i64, &[u8])] = &[
        (-7331, &[0xc5, 0x72]),
        (-65, &[0x81, 0x01]),
        (-64, &[0x7f]),
        (-2, &[0x03]),
        (-1, &[0x01]),
        (0, &[0x00]),
        (1, &[0x02]),
        (2, &[0x04]),
        (63, &[0x7e]),
        (64, &[0x80, 0x01]),
        (4223, &[0xfe, 0x41]),
    ];

    #[test]
    fn read_vari64() {
        for test in SIGNED {
            assert_eq!(
                test.0,
                super::read_vari64(&mut Bytes::from(test.1)).unwrap(),
                "{:?}",
                test
            );
        }
    }

    #[test]
    fn write_vari64() {
        for test in SIGNED {
            let mut buf = BytesMut::new();
            super::write_vari64(&mut buf, test.0);
            assert_eq!(test.1, &buf, "{:?}", test);
        }
    }

    #[test]
    fn read_varu64() {
        for test in UNSIGNED {
            assert_eq!(
                test.0,
                super::read_varu64(&mut Bytes::from(test.1)).unwrap(),
                "{:?}",
                test
            );
        }
    }

    #[test]
    fn write_varu64() {
        for test in UNSIGNED {
            let mut buf = BytesMut::new();
            super::write_varu64(&mut buf, test.0);
            assert_eq!(test.1, &buf, "{:?}", test);
        }
    }

    quickcheck! {
        fn varint_back_and_forth_i64(x: i64) -> bool {
            if x > i64::MAX / 2 || x < i64::MIN / 2{
                return true; // sorry, that won't work...
            }
            let mut buf = BytesMut::new();
            super::write_vari64(&mut buf, x);
            let y = super::read_vari64(&mut buf.freeze()).unwrap();
            dbg!(x, y);
            x == y
        }
    }

    quickcheck! {
        fn varint_back_and_forth_u64(x: u64) -> bool {
            let mut buf = BytesMut::new();
            super::write_varu64(&mut buf, x);
            let y = super::read_varu64(&mut buf.freeze()).unwrap();
            x == y
        }
    }
}
