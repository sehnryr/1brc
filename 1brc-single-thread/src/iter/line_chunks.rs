use std::io::{Read, Seek, SeekFrom};

const BUFFER_SIZE: usize = 1_000_000;

pub struct ToLineChunks<R> {
    reader: R,
    position: usize,
}

impl<R: Read + Seek> Iterator for ToLineChunks<R> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);

        self.reader
            .seek(SeekFrom::Start(self.position as u64))
            .unwrap();

        if self
            .reader
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

pub trait IterLineChunks {
    type Output;

    fn iter_line_chunks(self) -> Self::Output;
}

impl<R: Read + Seek> IterLineChunks for R {
    type Output = ToLineChunks<R>;

    fn iter_line_chunks(self) -> Self::Output {
        ToLineChunks {
            reader: self,
            position: 0,
        }
    }
}
