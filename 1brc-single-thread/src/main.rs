mod chunk_builder;
mod hash;
mod iter;
mod record;
mod util;

use crate::chunk_builder::ChunkBuilder;
use crate::iter::IterRawRecords;
use crate::record::Records;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");

    let chunks = ChunkBuilder::new(sample_file_path);

    let mut records = Records::new();

    let mut i = 0;
    while let Ok(chunk) = chunks.get_chunk(i, 1_000_000) {
        for record in chunk.iter_raw_records() {
            records.add(record);
        }
        i += 1;
    }

    for record in records.into_iter() {
        println!("{}", record);
    }

    Ok(())
}
