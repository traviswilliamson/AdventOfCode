use std::{collections::HashMap, error::Error};

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut first_group : Vec<i32> = Vec::new();
    let mut second_group : Vec<i32> = Vec::new();
    for line in _input.split("\n") {
        if line.len() > 0 {
            let mut nums = line.split_ascii_whitespace();
            first_group.push(nums.next().expect("NAN").parse()?);
            second_group.push(nums.next().expect("NAN").parse()?);
        }
    }

    first_group.sort();
    second_group.sort();

    let mut diff_sum = 0;
    for pair in first_group.iter().zip(second_group.iter()) {
        diff_sum += (pair.0 - pair.1).abs();
    }

    return Ok(diff_sum.to_string());
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut first_group : Vec<i32> = Vec::new();
    let mut second_group : HashMap<i32, i32> = HashMap::new();
    for line in _input.split("\n") {
        if line.len() > 0 {
            let mut nums = line.split_ascii_whitespace();
            first_group.push(nums.next().expect("NAN").parse()?);
            let second_entry = second_group.entry(nums.next().expect("NAN").parse()?).or_insert(0);
            *second_entry += 1;
        }
    }

    let mut similarity_score = 0;
    let default = 0;
    for key in first_group {
        similarity_score += key * second_group.get(&key).unwrap_or(&default);
    }

    return Ok(similarity_score.to_string());
}
