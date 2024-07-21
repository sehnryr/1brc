mod hash_table;
mod record;
mod util;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;

use crate::hash_table::{Hash, HashTable};
use crate::record::Record;
use crate::util::parse_temperature;

const BUFFER_SIZE: usize = 1_000_000;

fn find_offset<R: Read + Seek>(file: &mut R) -> usize {
    let mut offset = 0;
    let mut byte = [0u8; 1];

    loop {
        if let Err(_) = file.read_exact(&mut byte) {
            break;
        }
        if byte[0] == b'\n' {
            break;
        }
        file.seek(SeekFrom::Current(-2)).unwrap();
        offset += 1;
    }

    offset
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");

    let cpu_count: usize = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let thread_count = cpu_count.min(8);
    let (tx, rx) = std::sync::mpsc::channel::<HashTable>();

    let mut threads = Vec::with_capacity(thread_count);

    for thread_index in 0..thread_count {
        let tx = tx.clone();
        let sample_file_path = sample_file_path.clone();

        threads.push(thread::spawn(move || {
            let mut sample_file = File::open(sample_file_path).unwrap();

            let mut records = HashTable::new();
            let mut buffer = Vec::with_capacity(BUFFER_SIZE);

            let mut loop_count = 0;
            loop {
                let mut current_position: usize = 0;
                let mut line_start_position: usize = 0;
                let mut city_name_len: usize = 0;

                sample_file
                    .seek(SeekFrom::Start(
                        (BUFFER_SIZE * (thread_index + loop_count * thread_count)) as u64,
                    ))
                    .unwrap();

                let offset = if thread_index == 0 && loop_count == 0 {
                    0
                } else {
                    find_offset(&mut sample_file)
                };

                if sample_file
                    .by_ref()
                    .take(BUFFER_SIZE as u64 + offset as u64)
                    .read_to_end(&mut buffer)
                    .unwrap()
                    <= 0
                {
                    break;
                }

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

                    records.insert_or_update(city.hash(), temperature, || {
                        Record::new(std::str::from_utf8(city).unwrap(), temperature)
                    });

                    line_start_position = current_position + 1;
                    current_position += 1;
                }

                buffer.clear();
                loop_count += 1;
            }

            tx.send(records).unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let mut final_records = HashTable::new();

    while let Ok(table) = rx.try_recv() {
        for (key, record) in table {
            final_records.insert_or_merge(key, record);
        }
    }

    let mut records = final_records.into_iter().collect::<Vec<_>>();
    records.sort_by(|(_, record1), (_, record2)| record1.city.cmp(&record2.city));

    for (_, record) in records {
        println!("{}", record);
    }

    Ok(())
}
