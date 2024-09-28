use std::fs::File;
use std::io::Result;
use std::path::Path;

use crate::record::raw::RawRecord;
use crate::util::parse_temperature;

use super::chunk::Chunks;

pub struct SampleFile {
    chunks: Chunks,
    chunk: Vec<u8>,
    position: usize,
}

impl SampleFile {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(Self {
            chunks: Chunks::new(file),
            chunk: Vec::new(),
            position: 0,
        })
    }

    pub fn next(&mut self) -> Option<RawRecord> {
        if self.position >= self.chunk.len() {
            if let Some(chunk) = self.chunks.next() {
                self.chunk = chunk;
                self.position = 0;
            } else {
                return None;
            }
        }

        let chunk = &self.chunk[self.position..];

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
