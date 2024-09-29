mod chunk_builder;
mod hash;
mod iter;
mod record;
mod util;

use std::path::Path;
#[cfg(feature = "thread")]
use std::thread;

use crate::chunk_builder::ChunkBuilder;
use crate::iter::IterRawRecords;
use crate::record::Records;

fn get_records<P, F>(path: P, index: F) -> Records
where
    P: AsRef<Path>,
    F: Fn(usize) -> usize,
{
    let chunks = ChunkBuilder::new(path);
    let mut records = Records::new();

    let mut i = 0;
    while let Ok(chunk) = chunks.get_chunk(index(i), 1_000_000) {
        for record in chunk.iter_raw_records() {
            records.add(record);
        }
        i += 1;
    }

    records
}

#[cfg(feature = "thread")]
fn get_records_thread<P: AsRef<Path> + Clone + Send + 'static>(path: P) -> Records {
    let cpu_count: usize = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let thread_count = cpu_count.min(8);
    let (tx, rx) = std::sync::mpsc::channel::<Records>();

    let mut threads = Vec::with_capacity(thread_count);

    for thread_index in 0..thread_count {
        let tx = tx.clone();
        let path = path.clone();

        threads.push(thread::spawn(move || {
            let records = get_records(path, |i| thread_index + i * thread_count);
            tx.send(records).unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let mut records = Records::new();
    while let Ok(records_thread) = rx.try_recv() {
        records.merge(records_thread);
    }

    records
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");

    #[cfg(not(feature = "thread"))]
    let records = get_records(sample_file_path, |i| i);
    #[cfg(feature = "thread")]
    let records = get_records_thread(sample_file_path);

    for record in records.into_iter() {
        println!("{}", record);
    }

    Ok(())
}
