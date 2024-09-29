use std::vec::IntoIter;

use crate::hash::Hash;

use super::RawRecord;
use super::Record;

pub struct Records {
    buckets: Vec<Vec<(u16, Record)>>,
    size: usize,
}

impl Records {
    #[inline(always)]
    pub fn new() -> Self {
        let initial_size = 1 << 16;
        let mut buckets = Vec::with_capacity(initial_size);

        for _ in 0..initial_size {
            buckets.push(Vec::new());
        }

        Self { buckets, size: 0 }
    }

    #[inline(always)]
    pub fn add(&mut self, raw_record: RawRecord) {
        let hash = raw_record.hash();
        let index = hash as usize;

        if self.buckets[index].is_empty() {
            self.buckets[index].push((hash, Record::from(raw_record)));
            self.size += 1;
            return;
        }

        self.buckets[index]
            .iter_mut()
            .find(|(k, _)| *k == hash)
            .map(|(_, v)| v.add(raw_record.temperature()));
    }
}

impl IntoIterator for Records {
    type Item = Record;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut records = Vec::new();

        for bucket in self.buckets {
            for (_, record) in bucket {
                records.push(record);
            }
        }

        records.sort_by(|record1, record2| record1.city.cmp(&record2.city));

        records.into_iter()
    }
}