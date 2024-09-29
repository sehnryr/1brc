use std::io::{Error, ErrorKind, Read, Seek};

type Result<T> = std::result::Result<T, Error>;

pub trait SeekByteHelper {
    fn seek_byte(&mut self, byte: u8) -> Result<usize>;
}

impl<R: Read + Seek> SeekByteHelper for R {
    fn seek_byte(&mut self, byte: u8) -> Result<usize> {
        let mut byte_buf = [0u8; 1];
        let mut offset = 0;
        loop {
            match self.read(&mut byte_buf) {
                Ok(0) => return Err(Error::new(ErrorKind::UnexpectedEof, "reached end of file")),
                Err(_) => break,
                _ => (),
            }
            if byte_buf[0] == byte {
                break;
            }
            offset += 1;
        }
        Ok(offset)
    }
}
