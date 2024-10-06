use crate::util::FindByte;

macro_rules! ctoi {
    ($value:expr) => {
        ($value - b'0') as i32
    };
}

macro_rules! atoi {
    ($d2:expr, $d1:expr, $d0:expr) => {
        ctoi!($d2) * 100 + ctoi!($d1) * 10 + ctoi!($d0)
    };
    ($d1:expr, $d0:expr) => {
        ctoi!($d1) * 10 + ctoi!($d0)
    };
}

#[inline(always)]
fn parse_city_temperature(chunk: &[u8]) -> (&[u8], i32, usize) {
    // Since city names have at least 3 characters, we can skip the first 3 bytes.
    let city_len = 3 + chunk[3..].find_byte_index(b';');
    let city = &chunk[..city_len];

    let (temp, temp_len) = match &chunk[city_len + 1..] {
        [b'-', d2, d1, b'.', d0, ..] => (-atoi!(d2, d1, d0), 5),
        [b'-', d1, b'.', d0, ..] => (-atoi!(d1, d0), 4),
        [d2, d1, b'.', d0, ..] => (atoi!(d2, d1, d0), 4),
        [d1, b'.', d0, ..] => (atoi!(d1, d0), 3),
        _ => unreachable!(),
    };

    let read_len = city_len + temp_len + 2;
    (city, temp, read_len)
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
