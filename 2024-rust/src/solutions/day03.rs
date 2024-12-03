use std::error::Error;
use regex::Regex;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in pattern.captures_iter(&_input).map(|c| c.extract()) {
        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }
    Ok(sum.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for (substring, [x, y]) in pattern.captures_iter(&_input).map(|c| c.extract()) {
        if substring == "do()" {
            enabled = true;
        }
        else if substring == "don't()" {
            enabled = false;
        }
        else if enabled {
            sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }
    Ok(sum.to_string())
}
