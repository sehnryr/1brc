use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;

struct CityTemperatureData {
    name: Box<str>,
    pub min: f64,
    pub max: f64,
    pub sum_mean: f64,
    pub count: usize,
}

impl Hash for CityTemperatureData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for CityTemperatureData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CityTemperatureData {}

impl PartialOrd for CityTemperatureData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for CityTemperatureData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl CityTemperatureData {
    fn new_with_temperature(name: Box<str>, temperature: f64) -> Self {
        Self {
            name,
            min: temperature,
            max: temperature,
            sum_mean: temperature,
            count: 1,
        }
    }

    fn merge(&mut self, other: Self) {
        if other.min < self.min {
            self.min = other.min;
        }

        if other.max > self.max {
            self.max = other.max;
        }

        self.sum_mean += other.sum_mean;
        self.count += other.count;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_file_path = std::env::args().nth(1).expect("No file path provided");
    let sample_file = std::fs::File::open(sample_file_path)?;
    let reader = std::io::BufReader::new(sample_file);

    let mut cities_temperature_sum: HashSet<CityTemperatureData> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let (city, temperature) = line.split_once(';').expect("Invalid line format");
        let temperature = temperature
            .parse::<f64>()
            .expect("Invalid temperature format");

        let mut city_temperature_data =
            CityTemperatureData::new_with_temperature(city.into(), temperature);

        let old_city_temperature_data = cities_temperature_sum.take(&city_temperature_data);

        if let Some(old_city_temperature_data) = old_city_temperature_data {
            city_temperature_data.merge(old_city_temperature_data);
            cities_temperature_sum.insert(city_temperature_data);
        } else {
            cities_temperature_sum.insert(city_temperature_data);
        }
    }

    let mut cities_temperature_sum: Vec<_> = cities_temperature_sum.into_iter().collect();
    cities_temperature_sum.sort();

    for city_temperature_data in cities_temperature_sum {
        println!(
            "{};{:.1};{:.1};{:.1}",
            city_temperature_data.name,
            city_temperature_data.min,
            city_temperature_data.sum_mean / city_temperature_data.count as f64,
            city_temperature_data.max,
        );
    }

    Ok(())
}
