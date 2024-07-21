#[inline(always)]
fn temperature_from_digits(d2: u8, d1: u8, d0: u8) -> i64 {
    ((d0 - b'0') as u64 + (d1 - b'0') as u64 * 10 + (d2 - b'0') as u64 * 100) as i64
}

#[inline(always)]
pub fn parse_temperature(buffer_line: &[u8]) -> i64 {
    match &buffer_line {
        [b'-', d2, d1, _, d0] => -temperature_from_digits(*d2, *d1, *d0),
        [b'-', d1, _, d0] => -temperature_from_digits(b'0', *d1, *d0),
        [d2, d1, _, d0] => temperature_from_digits(*d2, *d1, *d0),
        [d1, _, d0] => temperature_from_digits(b'0', *d1, *d0),
        _ => unreachable!(),
    }
}

#[inline(always)]
pub fn hash(bytes: &[u8]) -> u64 {
    let mut hash = 0u64;

    for byte in bytes {
        hash = hash.rotate_left(3) ^ *byte as u64;
    }

    hash
}
