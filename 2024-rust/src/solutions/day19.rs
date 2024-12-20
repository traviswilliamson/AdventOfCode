use std::error::Error;
use memoize::memoize;
use rayon::prelude::*;

#[memoize(Ignore: patterns)]
fn buildable(patterns: &Vec<&str>, design: String) -> bool {
    for pattern in patterns {
        if pattern == &design {
            return true;
        }
        if design.starts_with(pattern) {
            if buildable(patterns, design[pattern.len()..design.len()].to_string()) {
                return true;
            }
        }
    }
    false
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut halves = _input.split("\n\n");
    let patterns: Vec<&str> = halves.next().unwrap().trim().split(", ").collect();
    let designs: Vec<&str> = halves.next().unwrap().split_terminator("\n").collect();
    let solutions = designs.par_iter().filter(|d| buildable(&patterns, d.to_string())).count();
    Ok(solutions.to_string())
}

#[memoize(Ignore: patterns)]
fn num_buildable(patterns: &Vec<&str>, design: String) -> u64 {
    let mut sum = 0;
    for pattern in patterns {
        if pattern == &design {
            sum += 1;
        }
        if design.starts_with(pattern) {
            sum += num_buildable(patterns, design[pattern.len()..design.len()].to_string());
        }
    }
    sum
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut halves = _input.split("\n\n");
    let patterns: Vec<&str> = halves.next().unwrap().trim().split(", ").collect();
    let designs: Vec<&str> = halves.next().unwrap().split_terminator("\n").collect();
    let solutions = designs.par_iter().map(|d| num_buildable(&patterns, d.to_string())).sum::<u64>();
    Ok(solutions.to_string())
}
