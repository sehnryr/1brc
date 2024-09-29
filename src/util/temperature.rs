#[inline(always)]
fn temperature_from_digits(d2: u8, d1: u8, d0: u8) -> i32 {
    ((d0 - b'0') as u16 + (d1 - b'0') as u16 * 10 + (d2 - b'0') as u16 * 100) as i32
}

#[inline(always)]
pub fn parse_temperature(buffer_line: &[u8]) -> i32 {
    match &buffer_line {
        [b'-', d2, d1, _, d0] => -temperature_from_digits(*d2, *d1, *d0),
        [b'-', d1, _, d0] => -temperature_from_digits(b'0', *d1, *d0),
        [d2, d1, _, d0] => temperature_from_digits(*d2, *d1, *d0),
        [d1, _, d0] => temperature_from_digits(b'0', *d1, *d0),
        _ => unreachable!(),
    }
}
