use std::{collections::HashMap, error::Error};

use rayon::prelude::*;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    // Read input
    let mut spot = (0, 0);
    let grid = _input.split_terminator("\n").enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, c)| {
            match c {
                '.' => 1,
                '#' => i32::MIN,
                'S' => {
                    spot = (i as i32, j as i32);
                    0
                },
                'E' => i32::MAX,
                _ => panic!("Bad input!")
            }
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    // println!("{:?}", spot);

    // Run path
    let mut visited: HashMap<(i32, i32), i32> = HashMap::from([(spot, 0)]);
    let mut time = 0;
    'outer: loop {
        time += 1;
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let adj = (spot.0 + d.0, spot.1 + d.1);
            if grid[adj.0 as usize][adj.1 as usize] != i32::MIN && !visited.contains_key(&adj) {
                spot = adj;
                // In the future, could update grid if wanted to optimize this
                visited.insert(adj, time);
                // println!("{:?}", spot);
                if grid[adj.0 as usize][adj.1 as usize] == i32::MAX {
                    break 'outer;
                }
                break;
            }
        }
    }

    // println!("{:?}", visited.len());
    // Find cheats > 100
    let num_saved = visited.par_iter().map(|first| {
        let mut num = 0;
        for second in &visited {
            if (first.0.0 - second.0.0).abs() + (first.0.1 - second.0.1).abs() == 2 {
                let saved_time = (second.1 - first.1).abs() - 2;
                if saved_time >= 100 {
                    num +=1;
                }
            }
        }
        num
    }).sum::<i32>() / 2;
    // println!("{:?}", num_saved);
    Ok(num_saved.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    // Read input
    let mut spot = (0, 0);
    let grid = _input.split_terminator("\n").enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, c)| {
            match c {
                '.' => 1,
                '#' => i32::MIN,
                'S' => {
                    spot = (i as i32, j as i32);
                    0
                },
                'E' => i32::MAX,
                _ => panic!("Bad input!")
            }
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    // println!("{:?}", spot);

    // Run path
    let mut visited: HashMap<(i32, i32), i32> = HashMap::from([(spot, 0)]);
    let mut time = 0;
    'outer: loop {
        time += 1;
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let adj = (spot.0 + d.0, spot.1 + d.1);
            if grid[adj.0 as usize][adj.1 as usize] != i32::MIN && !visited.contains_key(&adj) {
                spot = adj;
                // In the future, could update grid if wanted to optimize this
                visited.insert(adj, time);
                // println!("{:?}", spot);
                if grid[adj.0 as usize][adj.1 as usize] == i32::MAX {
                    break 'outer;
                }
                break;
            }
        }
    }

    // println!("{:?}", visited.len());
    // Find cheats > 100
    let num_saved = visited.par_iter().map(|first| {
        let mut num = 0;
        for second in &visited {
            let distance = (first.0.0 - second.0.0).abs() + (first.0.1 - second.0.1).abs();
            if distance <= 20 {
                let saved_time = (second.1 - first.1).abs() - distance;
                if saved_time >= 100 {
                    num +=1;
                }
            }
        }
        num
    }).sum::<i32>() / 2;
    // println!("{:?}", num_saved);
    Ok(num_saved.to_string())
}
