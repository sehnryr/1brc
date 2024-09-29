use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use crate::util::SeekByteHelper;

pub struct ChunkBuilder<P> {
    path: P,
}

impl<P: AsRef<Path>> ChunkBuilder<P> {
    pub fn new(path: P) -> Self {
        Self { path }
    }

    pub fn get_chunk(&self, index: usize, size: usize) -> Result<Vec<u8>, std::io::Error> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);

        // Find start of chunk
        let offset = index * size;
        let chunk_start_position = if offset == 0 {
            0
        } else {
            reader.seek(SeekFrom::Start(offset as u64))?;
            offset + reader.seek_byte(b'\n')? + 1
        };

        // Find end of chunk
        let offset = (index + 1) * size;
        reader.seek(SeekFrom::Start(offset as u64))?;
        let chunk_end_position = offset + reader.seek_byte(b'\n')? + 1;

        // Return chunk
        reader.seek(SeekFrom::Start(chunk_start_position as u64))?;

        let mut buffer = Vec::with_capacity(size);
        reader
            .take((chunk_end_position - chunk_start_position) as u64)
            .read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
