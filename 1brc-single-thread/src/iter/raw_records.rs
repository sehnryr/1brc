use crate::record::raw::RawRecord;
use crate::util::parse_temperature;

pub struct ToRawRecords<'a> {
    lines: &'a [u8],
    position: usize,
}

impl<'a> Iterator for ToRawRecords<'a> {
    type Item = RawRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.lines.len() {
            return None;
        }

        let chunk = &self.lines[self.position..];

        let mut city_len = 0;
        let mut line_len = 0;

        for (i, &byte) in chunk.iter().enumerate() {
            if byte == b';' {
                city_len = i;
            } else if byte == b'\n' {
                line_len = i;
                break;
            }
        }

        let city = &chunk[..city_len];
        let temperature = parse_temperature(&chunk[city_len + 1..line_len]);

        self.position += line_len + 1;

        return Some(RawRecord::new(city, temperature));
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
            lines: self,
            position: 0,
        }
    }
}
