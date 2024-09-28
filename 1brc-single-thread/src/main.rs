mod iter;
mod record;
mod util;

use std::fs::File;

use crate::iter::line_chunks::IterLineChunks;
use crate::iter::raw_records::IterRawRecords;
use crate::record::records::Records;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");

    let file = File::open(&sample_file_path)?;

    let mut records = Records::new();

    for line_chunk in file.iter_line_chunks() {
        for record in line_chunk.iter_raw_records() {
            records.add(record);
        }
    }

    for record in records.to_vec() {
        println!("{}", record);
    }

    Ok(())
}
