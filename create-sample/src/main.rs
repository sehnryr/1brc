use std::io::Write;

use create_sample::random::Random;
use create_sample::{CITY_COUNT, CITY_DATA};

const STANDARD_DEVIATION: f64 = 10.0;
const WRITE_BUFFER_LINES: u32 = 1_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    let number = std::env::args()
        .nth(1)
        .expect("usage: create-sample <number>")
        .parse::<u32>()?;

    let mut rng = Random::default();
    let mut expected_file = std::fs::File::create("expected.txt")?;
    let mut sample_file = std::fs::File::create("sample.txt")?;
    let mut sample_buffer = String::with_capacity(WRITE_BUFFER_LINES as usize * 20);
    let mut cities_min = vec![f64::MAX; CITY_COUNT];
    let mut cities_max = vec![f64::MIN; CITY_COUNT];
    let mut cities_average = vec![0.0; CITY_COUNT];
    let mut cities_count = vec![0; CITY_COUNT];

    for i in 0..number {
        let random_city_index = rng.gen_u64() as usize % CITY_COUNT;
        let (city, temperature) = CITY_DATA[random_city_index];
        let measurement = rng.gen_normal(temperature, STANDARD_DEVIATION);
        cities_average[random_city_index] += measurement;
        cities_count[random_city_index] += 1;

        if measurement < cities_min[random_city_index] {
            cities_min[random_city_index] = measurement;
        }

        if measurement > cities_max[random_city_index] {
            cities_max[random_city_index] = measurement;
        }

        if i % WRITE_BUFFER_LINES == 0 {
            sample_file.write_all(sample_buffer.as_bytes())?;
            sample_buffer.clear();
        }

        sample_buffer += &format!("{};{:.1}\n", city, measurement);
    }

    sample_file.write_all(sample_buffer.as_bytes())?;

    for (i, (city, _)) in CITY_DATA.iter().enumerate() {
        let count = &cities_count[i];
        let average = &cities_average[i];
        let min = &cities_min[i];
        let max = &cities_max[i];

        writeln!(
            expected_file,
            "{};{:.1};{:.1};{:.1}",
            city,
            min,
            average / *count as f64,
            max,
        )?;
    }

    println!("Created {} lines in {:?}", number, start_time.elapsed());

    Ok(())
}
