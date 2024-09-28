use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::BUFFER_SIZE;

pub struct Chunks {
    file: File,
    position: usize,
}

impl Chunks {
    pub fn new(file: File) -> Self {
        Chunks { file, position: 0 }
    }
}

impl Iterator for Chunks {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);

        self.file
            .seek(SeekFrom::Start(self.position as u64))
            .unwrap();

        if self
            .file
            .by_ref()
            .take(BUFFER_SIZE as u64)
            .read_to_end(&mut buffer)
            .unwrap()
            <= 0
        {
            return None;
        }

        let last_newline = buffer.iter().rposition(|&byte| byte == b'\n').unwrap();
        self.position += last_newline + 1;

        Some(buffer.drain(..last_newline + 1).collect())
    }
}
