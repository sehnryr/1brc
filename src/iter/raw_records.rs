use std::simd::{cmp::SimdPartialEq, u8x32};

use crate::record::RawRecord;

#[inline(always)]
pub fn find_idx(data: &[u8], needle: u8) -> usize {
    let simd_width = 32; // 256-bit wide SIMD register

    for (i, chunk) in data.chunks_exact(simd_width).enumerate() {
        // Load 32 bytes into a 256-bit SIMD register
        let chunk = u8x32::from_slice(chunk);

        // Compare each byte in the chunk with the needle
        let cmp = chunk.simd_eq(u8x32::splat(needle));

        // Convert the comparison result to a bitmask vector
        let masks = cmp.to_bitmask_vector();

        // Find the first non-zero mask which indicates a match
        for j in 0..simd_width {
            let mask = masks[j];
            if mask != 0 {
                // Calculate the position of the match
                return i * simd_width + j * 8 + mask.trailing_zeros() as usize;
            }
        }
    }

    // Handle the remaining bytes
    data.chunks_exact(simd_width)
        .remainder()
        .iter()
        .position(|&c| c == needle)
        .unwrap()
}

#[inline(always)]
pub fn temperature_from_digits(d2: &u8, d1: &u8, d0: &u8) -> i32 {
    let d2 = *d2 as u16;
    let d1 = *d1 as u16;
    let d0 = *d0 as u16;

    let temperature = d2 * 100 + d1 * 10 + d0;
    let normalized = temperature - 5328; // b'0' * 100 + b'0' * 10 + b'0'

    normalized as i32
}

#[inline(always)]
fn parse_record(chunk: &[u8]) -> (RawRecord, usize) {
    // Since city names have at least 3 characters, we can skip the first 3 bytes.
    let city_len = 3 + find_idx(&chunk[3..], b';');

    let city = &chunk[..city_len];

    let (temp, temp_len) = match &chunk[city_len + 1..] {
        [b'-', d2, d1, b'.', d0, ..] => (-temperature_from_digits(d2, d1, d0), 5),
        [b'-', d1, b'.', d0, ..] => (-temperature_from_digits(&b'0', d1, d0), 4),
        [d2, d1, b'.', d0, ..] => (temperature_from_digits(d2, d1, d0), 4),
        [d1, b'.', d0, ..] => (temperature_from_digits(&b'0', d1, d0), 3),
        _ => unreachable!(),
    };

    (RawRecord::new(city, temp), city_len + temp_len + 2)
}

pub struct ToRawRecords<'a> {
    chunk: &'a [u8],
    position: usize,
}

impl<'a> Iterator for ToRawRecords<'a> {
    type Item = RawRecord<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.chunk.len() {
            return None;
        }

        let chunk = &self.chunk[self.position..];
        let (record, len) = parse_record(chunk);

        self.position += len;

        Some(record)
    }
}

pub trait IterRawRecords {
    type Output;

    fn iter_raw_records(self) -> Self::Output;
}

impl<'a> IterRawRecords for &'a [u8] {
    type Output = ToRawRecords<'a>;

    #[inline(always)]
    fn iter_raw_records(self) -> Self::Output {
        ToRawRecords {
            chunk: self,
            position: 0,
        }
    }
}
