use std::{cmp::Reverse, collections::BinaryHeap, error::Error};

const OPEN: char = '.';
const WALL: char = '#';
const END: char = 'E';

struct Spot {
    cost: i32,
    wall: bool,
    target: bool,
    direction: (i32, i32)
}

fn adjacent_spots(spot: (usize, usize)) -> [((usize, usize), (i32, i32)); 4] {
    [((spot.0 + 1, spot.1), (1, 0)), ((spot.0 - 1, spot.1), (-1, 0)), ((spot.0, spot.1 + 1), (0, 1)), ((spot.0, spot.1 - 1), (0, -1))]
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid = _input.split_terminator("\n").map(|l| {
        l.chars().map(|c| match c {
            OPEN => Spot {
                cost: i32::MAX,
                wall: false,
                target: false,
                direction: (0, 0)
            },
            WALL => Spot {
                cost: i32::MAX,
                wall: true,
                target: false,
                direction: (0, 0)
            },
            END => Spot {
                cost: i32::MAX,
                wall: false,
                target: true,
                direction: (0, 0)
            },
            _ => Spot {
                cost: 0,
                wall: false,
                target: false,
                direction: (0, 1)
            }
        }).collect::<Vec<Spot>>()
    }).collect::<Vec<Vec<Spot>>>();
    let mut queue = BinaryHeap::from([(Reverse(0), (grid.len() - 2, 1 as usize))]);
    'outer: loop {
        let working_spot_location = queue.pop().unwrap().1;
        let working_spot_cost = grid[working_spot_location.0][working_spot_location.1].cost;
        let working_spot_direction = grid[working_spot_location.0][working_spot_location.1].direction;
        for (adjacent_spot_location, direction_traveled) in adjacent_spots(working_spot_location) {
            let adjacent_spot = &mut grid[adjacent_spot_location.0][adjacent_spot_location.1];
            if !adjacent_spot.wall {
                let mut potential_cost = working_spot_cost + 1;
                if working_spot_direction != direction_traveled {
                    potential_cost += 1000;
                }
                if potential_cost < adjacent_spot.cost {
                    adjacent_spot.cost = potential_cost;
                    adjacent_spot.direction = direction_traveled;
                    queue.push((Reverse(potential_cost), adjacent_spot_location));
                    if adjacent_spot.target {
                        break 'outer;
                    }
                }
            }
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            
        }
    }
    Ok(grid[1][grid[1].len() - 2].cost.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    return Err(Box::<dyn Error>::from("Not implemented!"));
}
