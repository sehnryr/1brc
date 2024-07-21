use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

struct Record {
    pub min: i64,
    pub max: i64,
    pub sum: i64,
    pub count: usize,
}

impl Default for Record {
    #[inline(always)]
    fn default() -> Self {
        Self {
            min: 0,
            max: 0,
            sum: 0,
            count: 0,
        }
    }
}

impl Record {
    #[inline(always)]
    fn add(&mut self, temperature: i64) {
        self.min = self.min.min(temperature);
        self.max = self.max.max(temperature);
        self.sum += temperature;
        self.count += 1;
    }
}

#[inline(always)]
fn temperature_from_digits(d2: u8, d1: u8, d0: u8) -> i64 {
    ((d0 - b'0') as u64 + (d1 - b'0') as u64 * 10 + (d2 - b'0') as u64 * 100) as i64
}

#[inline(always)]
fn parse_line(buffer_line: &[u8]) -> (&[u8], i64) {
    match &buffer_line {
        [city @ .., b';', b'-', d2, d1, _, d0] => (city, -temperature_from_digits(*d2, *d1, *d0)),
        [city @ .., b';', b'-', d1, _, d0] => (city, -temperature_from_digits(b'0', *d1, *d0)),
        [city @ .., b';', d2, d1, _, d0] => (city, temperature_from_digits(*d2, *d1, *d0)),
        [city @ .., b';', d1, _, d0] => (city, temperature_from_digits(b'0', *d1, *d0)),
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");
    let mut sample_file = BufReader::new(File::open(sample_file_path)?);
    let mut buffer = Vec::with_capacity(10_000_000);
    let mut buffer_line = Vec::with_capacity(20);

    let mut records: HashMap<Box<[u8]>, RefCell<Record>> = HashMap::new();

    while sample_file
        .by_ref()
        .take(10_000_000)
        .read_to_end(&mut buffer)?
        > 0
    {
        let mut buffer_iter = buffer.iter();

        while let Some(&byte) = buffer_iter.next() {
            if byte != b'\n' {
                buffer_line.push(byte);
                continue;
            }

            let (city, temperature) = parse_line(&buffer_line);

            let record = records
                .entry(city.into())
                .or_insert_with(|| RefCell::new(Record::default()));

            record.borrow_mut().add(temperature);

            buffer_line.clear();
        }

        buffer.clear();
    }

    let mut records = records
        .into_iter()
        .map(|(name, record)| (name, record.into_inner()))
        .collect::<Vec<_>>();
    records.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

    for (city, record) in records {
        println!(
            "{};{:.1};{:.1};{:.1}",
            std::str::from_utf8(&city)?,
            record.min as f64 / 10.0,
            record.sum as f64 / 10.0 / record.count as f64,
            record.max as f64 / 10.0,
        );
    }

    Ok(())
}
