mod file;
mod record;
mod util;

use crate::file::sample::SampleFile;
use crate::record::map::RecordMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");

    let mut sample_file = SampleFile::open(sample_file_path)?;

    let mut records = RecordMap::new();

    while let Some(record) = sample_file.next() {
        records.add(record);
    }

    for record in records.to_vec() {
        println!("{}", record);
    }

    Ok(())
}
