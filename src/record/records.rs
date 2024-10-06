use std::vec::IntoIter;

use crate::hash::Hash;

use super::Record;

pub struct Records {
    records: Vec<Option<Record>>,
}

impl Records {
    #[inline(always)]
    pub fn new() -> Self {
        let initial_size = 1 << 16;
        let mut records = Vec::with_capacity(initial_size);
        records.resize_with(initial_size, || None);

        Self { records }
    }

    #[inline(always)]
    pub fn add(&mut self, city: &[u8], temperature: i32) {
        let hash = city.hash();
        let record = &mut self.records[hash as usize];

        if let Some(record) = record {
            record.add(temperature);
        } else {
            *record = Some(Record::new(city, temperature));
        }
    }

    #[cfg(feature = "thread")]
    #[inline(always)]
    pub fn merge(&mut self, other: Records) {
        for (index, record) in other.records.into_iter().enumerate() {
            if let Some(record) = record {
                if let Some(self_record) = &mut self.records[index] {
                    self_record.merge(record);
                } else {
                    self.records.insert(index, Some(record));
                }
            }
        }
    }
}

impl IntoIterator for Records {
    type Item = Record;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut records = Vec::new();

        for bucket in self.records {
            if let Some(record) = bucket {
                records.push(record);
            }
        }

        records.sort_by(|record1, record2| {
            let city1 = record1.city();
            let city2 = record2.city();

            city1.cmp(city2)
        });

        records.into_iter()
    }
}
