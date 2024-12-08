use std::{collections::{HashMap, HashSet}, error::Error};

use itertools::Itertools;

const LEN: i32 = 50;
const HEIGHT: i32 = 50;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n")
        .enumerate()
        .fold(HashMap::new(), |mut map: HashMap<char, HashSet<(i32, i32)>>, (i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c != '.' {
                    if !map.contains_key(&c) {
                        map.insert(c, HashSet::new());
                    }
                    map.get_mut(&c).unwrap().insert((i as i32, j as i32));
                }
            });
            map
        })
        .iter()
        .fold(HashSet::new(), |mut set: HashSet<(i32, i32)>, frequecy| {
            frequecy.1.iter().tuple_combinations::<(&(i32, i32), &(i32, i32))>()
                .for_each(|combo| {
                    let diff = (combo.1.0 - combo.0.0, combo.1.1 - combo.0.1);
                    let near = (combo.0.0 - diff.0, combo.0.1 - diff.1);
                    if near.0 >= 0 && near.0 < HEIGHT && near.1 >= 0 && near.1 < LEN {
                        set.insert(near);
                    }
                    let far = (combo.1.0 + diff.0, combo.1.1 + diff.1);
                    if far.0 >= 0 && far.0 < HEIGHT && far.1 >= 0 && far.1 < LEN {
                        set.insert(far);
                    }
                });
            set
        })
        .iter().count().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n")
        .enumerate()
        .fold(HashMap::new(), |mut map: HashMap<char, HashSet<(i32, i32)>>, (i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c != '.' {
                    if !map.contains_key(&c) {
                        map.insert(c, HashSet::new());
                    }
                    map.get_mut(&c).unwrap().insert((i as i32, j as i32));
                }
            });
            map
        })
        .iter()
        .fold(HashSet::new(), |mut set: HashSet<(i32, i32)>, frequecy| {
            frequecy.1.iter().tuple_combinations::<(&(i32, i32), &(i32, i32))>()
                .for_each(|combo| {
                    let diff = (combo.1.0 - combo.0.0, combo.1.1 - combo.0.1);
                    let mut spot = combo.0.clone();
                    while spot.0 >= 0 && spot.0 < HEIGHT && spot.1 >= 0 && spot.1 < LEN {
                        set.insert(spot);
                        spot = (spot.0 - diff.0, spot.1 - diff.1);
                    }
                    spot = combo.1.clone();
                    while spot.0 >= 0 && spot.0 < HEIGHT && spot.1 >= 0 && spot.1 < LEN {
                        set.insert(spot);
                        spot = (spot.0 + diff.0, spot.1 + diff.1);
                    }
                });
            set
        })
        .iter().count().to_string())
}
