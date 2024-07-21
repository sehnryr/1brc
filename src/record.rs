pub struct Record {
    pub city: Box<str>,
    pub min: i64,
    pub max: i64,
    pub sum: i64,
    pub count: usize,
}

impl std::fmt::Display for Record {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{:.1};{:.1};{:.1}",
            self.city,
            self.min as f64 / 10.0,
            self.sum as f64 / 10.0 / self.count as f64,
            self.max as f64 / 10.0,
        )
    }
}

impl Record {
    #[inline(always)]
    pub fn new(city: &str, temperature: i64) -> Self {
        Self {
            city: Box::from(city),
            min: temperature,
            max: temperature,
            sum: temperature,
            count: 1,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, temperature: i64) {
        if temperature < self.min {
            self.min = temperature;
        }

        if temperature > self.max {
            self.max = temperature;
        }

        self.sum += temperature;
        self.count += 1;
    }

    #[inline(always)]
    pub fn merge(&mut self, other: Self) {
        if other.min < self.min {
            self.min = other.min;
        }

        if other.max > self.max {
            self.max = other.max;
        }

        self.sum += other.sum;
        self.count += other.count;
    }
}
