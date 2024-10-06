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
        let bucket = &mut self.buckets[hash as usize];

        if bucket.is_empty() {
            bucket.push((hash, Record::from(raw_record)));
            self.size += 1;
            return;
        }

        let record = match bucket.len() {
            1 => bucket.get_mut(0),
            _ => bucket.iter_mut().find(|(k, _)| *k == hash),
        };
        record.map(|(_, v)| v.add(raw_record.temperature));
    }

    #[cfg(feature = "thread")]
    #[inline(always)]
    pub fn merge(&mut self, other: Records) {
        for bucket in other.buckets {
            for (hash, record) in bucket {
                if self.buckets[hash as usize]
                    .iter_mut()
                    .find(|(k, _)| *k == hash)
                    .map(|(_, v)| v)
                    .is_some()
                {
                    self.buckets[hash as usize]
                        .iter_mut()
                        .find(|(k, _)| *k == hash)
                        .map(|(_, v)| v.merge(record));
                } else {
                    self.buckets[hash as usize].push((hash, record));
                    self.size += 1;
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

        for bucket in self.buckets {
            for (_, record) in bucket {
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
