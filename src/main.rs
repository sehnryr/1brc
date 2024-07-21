mod record;
mod util;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::record::Record;
use crate::util::{hash, parse_temperature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");
    let mut sample_file = BufReader::new(File::open(sample_file_path)?);
    let mut buffer = Vec::with_capacity(10_000_000);
    let mut old_buffer_part = Vec::new();
    let mut current_position: usize = 0;
    let mut line_start_position: usize = 0;
    let mut city_name_len: usize = 0;

    let mut records: HashMap<u64, Record> = HashMap::new();

    while sample_file
        .by_ref()
        .take(10_000_000)
        .read_to_end(&mut buffer)?
        > 0
    {
        buffer = [old_buffer_part.as_slice(), &buffer[..]].concat();
        for &byte in &buffer {
            if byte == b';' {
                city_name_len = current_position - line_start_position;
                current_position += 1;
                continue;
            }
            if byte != b'\n' {
                current_position += 1;
                continue;
            }

            let city = &buffer[line_start_position..line_start_position + city_name_len];
            let temperature = parse_temperature(
                &buffer[line_start_position + city_name_len + 1..current_position],
            );

            records
                .entry(hash(city))
                .and_modify(|record| record.add(temperature))
                .or_insert_with(|| Record::new(std::str::from_utf8(city).unwrap(), temperature));

            line_start_position = current_position + 1;
            current_position += 1;
        }

        old_buffer_part = buffer[line_start_position..].to_vec();
        buffer.clear();
        current_position = 0;
        line_start_position = 0;
    }

    println!("{}", records.len());

    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by(|(_, record1), (_, record2)| record1.city.cmp(&record2.city));

    for (_, record) in records {
        println!("{}", record);
    }

    Ok(())
}
