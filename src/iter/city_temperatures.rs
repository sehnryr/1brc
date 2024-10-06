use crate::util::FindByte;

#[inline(always)]
pub fn temperature_from_digits(d2: &u8, d1: &u8, d0: &u8) -> i32 {
    let d2 = *d2 as u16;
    let d1 = *d1 as u16;
    let d0 = *d0 as u16;

    let temperature = d2 * 100 + d1 * 10 + d0;
    let normalized = temperature - 5328; // b'0' * 100 + b'0' * 10 + b'0'

    normalized as i32
}

#[inline(always)]
fn parse_city_temperature(chunk: &[u8]) -> (&[u8], i32, usize) {
    // Since city names have at least 3 characters, we can skip the first 3 bytes.
    let city_len = 3 + &chunk[3..].find_byte_index(b';');

    let city = &chunk[..city_len];

    let (temp, temp_len) = match &chunk[city_len + 1..] {
        [b'-', d2, d1, b'.', d0, ..] => (-temperature_from_digits(d2, d1, d0), 5),
        [b'-', d1, b'.', d0, ..] => (-temperature_from_digits(&b'0', d1, d0), 4),
        [d2, d1, b'.', d0, ..] => (temperature_from_digits(d2, d1, d0), 4),
        [d1, b'.', d0, ..] => (temperature_from_digits(&b'0', d1, d0), 3),
        _ => unreachable!(),
    };

    (city, temp, city_len + temp_len + 2)
}

pub struct ToCityTemperatures<'a> {
    chunk: &'a [u8],
    position: usize,
}

impl<'a> Iterator for ToCityTemperatures<'a> {
    type Item = (&'a [u8], i32);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.chunk.len() {
            return None;
        }

        let chunk = &self.chunk[self.position..];
        let (city, temp, len) = parse_city_temperature(chunk);

        self.position += len;

        Some((city, temp))
    }
}

pub trait IterCityTemperatures {
    type Output;

    fn iter_city_temperatures(self) -> Self::Output;
}

impl<'a> IterCityTemperatures for &'a [u8] {
    type Output = ToCityTemperatures<'a>;

    #[inline(always)]
    fn iter_city_temperatures(self) -> Self::Output {
        ToCityTemperatures {
            chunk: self,
            position: 0,
        }
    }
}