use std::{collections::HashMap, error::Error};

use itertools::Position;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid: HashMap<(i32, i32), char> = _input.split("\n").enumerate().map(|l| l.1.chars().enumerate().map(move |c| ((l.0 as i32, c.0 as i32), c.1))).flatten().collect();
    let mut position = *grid.iter().find(|e| e.1 == &'^').unwrap().0;
    let mut direction = (-1,0);
    let right_turns = [(-1,0), (0,1), (1,0), (0,-1)];
    while grid.contains_key(&position) {
        *grid.get_mut(&position).unwrap() = 'X';
        let next = (position.0 + direction.0, position.1 + direction.1);
        if grid.contains_key(&next) {
            if grid.get(&next).unwrap() == &'#' {
                direction = right_turns[(right_turns.iter().position(|d| *d == direction).unwrap() + 1) % 4];
            }
            else {
                position = next;
            }
        }
        else {
            position = next;
        }
    }
    Ok(grid.into_values().filter(|c| c == &'X').count().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid: HashMap<(i32, i32), (char, Option<(i32,i32)>, bool)> = _input.split("\n").enumerate().map(|l| l.1.chars().enumerate().map(move |c| ((l.0 as i32, c.0 as i32), (c.1, None, true)))).flatten().collect();
    let mut position = *grid.iter().find(|e| e.1.0 == '^').unwrap().0;
    grid.get_mut(&position).unwrap().2 = false;
    let mut direction = (-1,0);
    let right_turns = [(-1,0), (0,1), (1,0), (0,-1)];
    while grid.contains_key(&position) {
        grid.get_mut(&position).unwrap().0 = 'X';
        grid.get_mut(&position).unwrap().1 = Some(direction);
        let next = (position.0 + direction.0, position.1 + direction.1);
        if grid.contains_key(&next) {
            if grid.get(&next).unwrap().0 == '#' {
                direction = right_turns[(right_turns.iter().position(|d| *d == direction).unwrap() + 1) % 4];
            }
            else {
                position = next;
            }
        }
        else {
            position = next;
        }
    }
    grid.iter().filter(|e| e.1.0 == 'X' && e.1.2).map(|e| e.0).collect();
    Ok(grid.into_values().filter(|c| c.0 == 'X').count().to_string())
}
