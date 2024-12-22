use std::{collections::HashMap, error::Error};

use rayon::prelude::*;


pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n").par_bridge().map(|l| {
        let mut s = l.parse::<u64>().unwrap(); // Remember, 16777216 is 2**24
        for _ in 0..2000 {
            s = ((s << 6) ^ s) % 16777216;
            s = ((s >> 5) ^ s) % 16777216;
            s = ((s << 11) ^ s) % 16777216;
        }
        s
    }).sum::<u64>().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let changes_to_total_price = _input.split_terminator("\n").par_bridge().map(|l| {
        let mut secret = l.parse::<i64>().unwrap(); // Remember, 16777216 is 2**24
        let mut ones_digit = (secret % 10) as i8;
        let mut changes = (i8::MAX, i8::MAX, i8::MAX, i8::MAX);
        let mut changes_to_price = HashMap::new();
        for _ in 0..2000 {
            secret = ((secret << 6) ^ secret) % 16777216;
            secret = ((secret >> 5) ^ secret) % 16777216;
            secret = ((secret << 11) ^ secret) % 16777216;
            let change = ((secret % 10) as i8) - ones_digit;
            ones_digit = (secret % 10) as i8;
            (changes.0, changes.1, changes.2, changes.3) = (changes.1, changes.2, changes.3, change);
            if changes.0 != i8::MAX && !changes_to_price.contains_key(&changes) {
                changes_to_price.insert(changes, ones_digit as i32);
            }
        }
        changes_to_price
    }).reduce(|| HashMap::new(), |mut existing_map, new_map| {
        for (k, v) in new_map {
            let e = existing_map.entry(k).or_insert(0);
            *e += v;
        }
        existing_map
    });
    Ok(changes_to_total_price.into_values().par_bridge().max().unwrap().to_string())
}
