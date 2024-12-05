use std::error::Error;
use itertools::Itertools;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let grid : Vec<Vec<char>> = _input.split("\n").map(|s| s.chars().collect()).collect();
    let grid_height = 0..(grid.len() as i32);
    let grid_len = 0..(grid[0].len() as i32);
    let directions: Vec<(i32, i32)> = (-1..2).cartesian_product(-1..2).collect();
    let mut count = 0;
    for (i, j) in grid_height.clone().cartesian_product(grid_len.clone()) {
        for direction in directions.iter() {
            let mut word = String::from("");
            for index in 0..4 {
                let i : i32 = i + (direction.0 * index);
                let j : i32 = j + (direction.1 * index);
                if grid_height.contains(&i) && grid_len.contains(&j) {
                    word.push(grid[i as usize][j as usize]);
                }
            }
            if word == "XMAS" {
                count += 1;
            }
        }
    }
    return Ok(count.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let grid : Vec<Vec<char>> = _input.split("\n").map(|s| s.chars().collect()).collect();
    let grid_height = 1..((grid.len() as i32)-1);
    let grid_len = 1..((grid[0].len() as i32)-1);
    let directions: Vec<(&i32, &i32)> = [-1,1].iter().cartesian_product([-1,1].iter()).collect();
    let words = ["MSMS", "MMSS", "SSMM", "SMSM"];
    let mut count = 0;
    for (i, j) in grid_height.clone().cartesian_product(grid_len.clone()) {
        if grid[i as usize][j as usize] == 'A' {
            let mut word = String::from("");
            for direction in directions.iter() {
                let i : i32 = i + direction.0;
                let j : i32 = j + direction.1;
                word.push(grid[i as usize][j as usize]);
            }
            if words.contains(&word.as_str()) {
                count += 1;
            }
        }
    }
    return Ok(count.to_string())
}
