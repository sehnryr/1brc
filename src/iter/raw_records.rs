use crate::record::RawRecord;
use crate::util::parse_temperature;

pub struct ToRawRecords<'a> {
    chunk: &'a [u8],
    position: usize,
}

impl<'a> Iterator for ToRawRecords<'a> {
    type Item = RawRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.chunk.len() {
            return None;
        }

        let chunk = &self.chunk[self.position..];

        // Since city names have at least 3 characters, we can skip the first 3 bytes.
        // Same for the temperature, but we also need to skip the semicolon.
        let city_len = 3 + chunk[3..].iter().position(|&c| c == b';')?;
        let temp_len = 3 + chunk[city_len + 4..].iter().position(|&c| c == b'\n')?;

        let city = &chunk[..city_len];
        let temperature = parse_temperature(&chunk[city_len + 1..city_len + 1 + temp_len]);

        self.position += city_len + temp_len + 2;

        Some(RawRecord::new(city, temperature))
    }
}

pub trait IterRawRecords {
    type Output;

    fn iter_raw_records(self) -> Self::Output;
}

impl<'a> IterRawRecords for &'a [u8] {
    type Output = ToRawRecords<'a>;

    fn iter_raw_records(self) -> Self::Output {
        ToRawRecords {
            chunk: self,
            position: 0,
        }
    }
}
