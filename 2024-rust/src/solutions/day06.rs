use std::{collections::HashMap, error::Error};

use rayon::prelude::*;

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

fn char_to_int (_char : char) -> i32 {
    match _char {
        '^' => -1,
        '#' => -2,
        'X' => -3,
        _ => -4
    }
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid: HashMap<(i32, i32), (i32, Option<(i32,i32)>, bool)> = _input.split("\n").enumerate().map(|l| l.1.chars().enumerate().map(move |c| ((l.0 as i32, c.0 as i32), (char_to_int(c.1), None, true)))).flatten().collect();
    let mut position = *grid.iter().find(|e| e.1.0 == char_to_int('^')).unwrap().0;
    let start_position = position.clone();
    grid.get_mut(&position).unwrap().2 = false;
    let mut direction = (-1,0);
    let right_turns = [(-1,0), (0,1), (1,0), (0,-1)];
    while grid.contains_key(&position) {
        let current = grid.get_mut(&position).unwrap();
        current.0 = char_to_int('X');
        current.1 = Some(direction);
        let next = (position.0 + direction.0, position.1 + direction.1);
        if grid.contains_key(&next) {
            if grid.get(&next).unwrap().0 == char_to_int('#') {
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
    let spots_to_check: Vec<(i32, i32)> = grid.clone().into_iter().filter(|c| c.1.0 == char_to_int('X') && c.1.2).map(|e| e.0).collect();
    Ok(spots_to_check
        .par_iter()
        .enumerate()
        .filter(|s| {
            let mut thread_grid = grid.clone();
            let mut thread_position = start_position.clone();
            let mut thread_direction = (-1,0);
            thread_grid.get_mut(&s.1).unwrap().0 = char_to_int('#');
            while thread_grid.contains_key(&thread_position) {
                let thread_current = thread_grid.get_mut(&thread_position).unwrap();
                if thread_current.0 == s.0 as i32 && thread_current.1 == Some(thread_direction) {
                    thread_grid.get_mut(&s.1).unwrap().0 = char_to_int('.');
                    return true;
                }
                if thread_current.1 == None || thread_current.0 != s.0 as i32 {
                    thread_current.1 = Some(thread_direction);
                }
                thread_current.0 = s.0 as i32;
                let thread_next = (thread_position.0 + thread_direction.0, thread_position.1 + thread_direction.1);
                if thread_grid.contains_key(&thread_next) {
                    if thread_grid.get(&thread_next).unwrap().0 == char_to_int('#') {
                        thread_direction = right_turns[(right_turns.iter().position(|d| *d == thread_direction).unwrap() + 1) % 4];
                    }
                    else {
                        thread_position = thread_next;
                    }
                }
                else {
                    thread_position = thread_next;
                }
            }
            thread_grid.get_mut(&s.1).unwrap().0 = char_to_int('.');
            false
        })
        .count().to_string())
}
