use std::collections::{HashMap, HashSet};

use create_sample::CITY_DATA;
use itertools::iproduct;

fn key_elem(city: &str, mask: u8) -> Vec<u8> {
    let bytes = city.as_bytes();
    let len = bytes.len();
    let mut key = Vec::new();

    if mask & 0b1 != 0 {
        key.push(len as u8);
    }
    if mask & 0b10 != 0 {
        key.push(bytes[0]);
    }
    if mask & 0b100 != 0 {
        key.push(bytes[1]);
    }
    if mask & 0b1000 != 0 {
        key.push(bytes[2]);
    }
    if mask & 0b10000 != 0 {
        key.push(bytes[len - 3]);
    }
    if mask & 0b100000 != 0 {
        key.push(bytes[len - 2]);
    }
    if mask & 0b1000000 != 0 {
        key.push(bytes[len - 1]);
    }
    key
}

fn main() {
    let cities = CITY_DATA.iter().map(|(city, _)| city).collect::<Vec<_>>();
    let length = cities.len();

    // for (a, b, c, d) in iproduct!(0..16, 0..16, 0..16, 0..16) {
    //     let mut set = HashSet::new();
    //     for city in &cities {
    //         let bytes = city.as_bytes();
    //         let mut hash = bytes.len() as u16;
    //         hash = hash.rotate_left(a) ^ bytes[0] as u16;
    //         hash = hash.rotate_left(b) ^ bytes[1] as u16;
    //         hash = hash.rotate_left(c) ^ bytes[2] as u16;
    //         hash = hash.rotate_left(d) ^ bytes[bytes.len() - 3] as u16;
    //         if !set.insert(hash) {
    //             continue;
    //         }
    //     }
    //     if set.len() == length {
    //         println!("Hash: {} {} {} {}", a, b, c, d);
    //     }
    // }

    'mask_loop: for mask in 0b0..0b10000000 {
        let mut map = HashMap::new();
        for city in &cities {
            let key = key_elem(city, mask);

            if map.contains_key(&key) {
                continue 'mask_loop;
            } else {
                map.insert(key, city);
            }
        }
        println!("Mask: {:07b}", mask);
    }
}
