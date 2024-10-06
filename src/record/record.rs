pub struct Record {
    city: Box<[u8]>,
    min: i32,
    max: i32,
    sum: i32,
    count: usize,
}

impl std::fmt::Display for Record {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{:.1};{:.1};{:.1}",
            std::str::from_utf8(&self.city).unwrap(),
            self.min as f64 / 10.0,
            self.sum as f64 / 10.0 / self.count as f64,
            self.max as f64 / 10.0,
        )
    }
}

impl Record {
    #[inline(always)]
    pub fn city(&self) -> &[u8] {
        &self.city
    }

    #[inline(always)]
    pub fn new(city: &[u8], temperature: i32) -> Self {
        Self {
            city: Box::from(city),
            min: temperature,
            max: temperature,
            sum: temperature,
            count: 1,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, temperature: i32) {
        if temperature < self.min {
            self.min = temperature;
        } else if temperature > self.max {
            self.max = temperature;
        }

        self.sum += temperature;
        self.count += 1;
    }

    #[cfg(feature = "thread")]
    #[inline(always)]
    pub fn merge(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}
