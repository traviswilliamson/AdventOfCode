use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, error::Error};

const OPEN: char = '.';
const WALL: char = '#';
const END: char = 'E';
const START: char = 'S';

#[derive(Clone)]
struct Spot {
    cost: i32,
    wall: bool,
    direction: (i32, i32),
    path: HashSet<(usize, usize)>
}

fn adjacent_spots(spot: (usize, usize)) -> [((usize, usize), (i32, i32)); 4] {
    [((spot.0 + 1, spot.1), (1, 0)), ((spot.0 - 1, spot.1), (-1, 0)), ((spot.0, spot.1 + 1), (0, 1)), ((spot.0, spot.1 - 1), (0, -1))]
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid = _input.split_terminator("\n").map(|l| {
        l.chars().filter(|c| *c != '\r').map(|c| match c {
            OPEN => Spot {
                cost: i32::MAX,
                wall: false,
                direction: (0, 0),
                path: HashSet::new()
            },
            WALL => Spot {
                cost: i32::MAX,
                wall: true,
                direction: (0, 0),
                path: HashSet::new()
            },
            END => Spot {
                cost: i32::MAX,
                wall: false,
                direction: (0, 0),
                path: HashSet::new()
            },
            START => Spot {
                cost: 0,
                wall: false,
                direction: (0, 1),
                path: HashSet::new()
            },
            x => panic!("FOUND {:?}", x)
        }).collect::<Vec<Spot>>()
    }).collect::<Vec<Vec<Spot>>>();
    let mut queue = BinaryHeap::from([(Reverse(0), (grid.len() - 2, 1 as usize))]);
    let end = (1 as usize, grid[1].len() - 2);
    'outer: while queue.len() > 0 {
        let working_spot_location = queue.pop().unwrap().1;
        let working_spot_cost = grid[working_spot_location.0][working_spot_location.1].cost;
        let working_spot_direction = grid[working_spot_location.0][working_spot_location.1].direction;
        for (adjacent_spot_location, direction_traveled) in adjacent_spots(working_spot_location) {
            let adjacent_spot_wall = grid[adjacent_spot_location.0][adjacent_spot_location.1].wall;
            let adjacent_spot_cost = grid[adjacent_spot_location.0][adjacent_spot_location.1].cost;
            let adjacent_spot_direction = grid[adjacent_spot_location.0][adjacent_spot_location.1].direction;
            if !adjacent_spot_wall {
                let mut potential_cost = working_spot_cost + 1;
                if working_spot_direction != direction_traveled {
                    potential_cost += 1000;
                }
                let spot_beyond_cost = grid[working_spot_location.0 + ((adjacent_spot_location.0 - working_spot_location.0) * 2)][working_spot_location.1 + ((adjacent_spot_location.1 - working_spot_location.1) * 2)].cost;
                let spot_beyond_wall = grid[working_spot_location.0 + ((adjacent_spot_location.0 - working_spot_location.0) * 2)][working_spot_location.1 + ((adjacent_spot_location.1 - working_spot_location.1) * 2)].wall;
                if potential_cost < adjacent_spot_cost
                || (adjacent_spot_direction != direction_traveled && (potential_cost + 1) < spot_beyond_cost && !spot_beyond_wall){
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].cost = potential_cost;
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].direction = direction_traveled;
                    queue.push((Reverse(potential_cost), adjacent_spot_location));
                    if adjacent_spot_location == end {
                        break 'outer;
                    }
                }
            }
        }
    }
    Ok(grid[1][grid[1].len() - 2].cost.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid = _input.split_terminator("\n").map(|l| {
        l.chars().filter(|c| *c != '\r').map(|c| match c {
            OPEN => Spot {
                cost: i32::MAX,
                wall: false,
                direction: (0, 0),
                path: HashSet::new()
            },
            WALL => Spot {
                cost: i32::MAX,
                wall: true,
                direction: (0, 0),
                path: HashSet::new()
            },
            END => Spot {
                cost: i32::MAX,
                wall: false,
                direction: (0, 0),
                path: HashSet::new()
            },
            START => Spot {
                cost: 0,
                wall: false,
                direction: (0, 1),
                path: HashSet::from([(l.len() - 2, 1 as usize)])
            },
            x => panic!("FOUND {:?}", x)
        }).collect::<Vec<Spot>>()
    }).collect::<Vec<Vec<Spot>>>();
    let mut queue = BinaryHeap::from([(Reverse(0), (grid.len() - 2, 1 as usize))]);
    let end = (1 as usize, grid[1].len() - 2);
    while queue.len() > 0 {
        let working_spot_location = queue.pop().unwrap().1;
        let working_spot_cost = grid[working_spot_location.0][working_spot_location.1].cost;
        let working_spot_direction = grid[working_spot_location.0][working_spot_location.1].direction;
        for (adjacent_spot_location, direction_traveled) in adjacent_spots(working_spot_location) {
            let adjacent_spot_wall = grid[adjacent_spot_location.0][adjacent_spot_location.1].wall;
            let adjacent_spot_cost = grid[adjacent_spot_location.0][adjacent_spot_location.1].cost;
            let adjacent_spot_direction = grid[adjacent_spot_location.0][adjacent_spot_location.1].direction;
            if !adjacent_spot_wall {
                let mut potential_cost = working_spot_cost + 1;
                if working_spot_direction != direction_traveled {
                    potential_cost += 1000;
                }
                let spot_beyond_cost = grid[working_spot_location.0 + ((adjacent_spot_location.0 - working_spot_location.0) * 2)][working_spot_location.1 + ((adjacent_spot_location.1 - working_spot_location.1) * 2)].cost;
                let spot_beyond_wall = grid[working_spot_location.0 + ((adjacent_spot_location.0 - working_spot_location.0) * 2)][working_spot_location.1 + ((adjacent_spot_location.1 - working_spot_location.1) * 2)].wall;
                if potential_cost < adjacent_spot_cost
                || (adjacent_spot_direction != direction_traveled && (potential_cost + 1) < spot_beyond_cost && !spot_beyond_wall && spot_beyond_cost != i32::MAX) {
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].cost = potential_cost;
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].direction = direction_traveled;
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].path = grid[working_spot_location.0][working_spot_location.1].path.clone();
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].path.insert(adjacent_spot_location);
                    queue.push((Reverse(potential_cost), adjacent_spot_location));
                }
                else if potential_cost == adjacent_spot_cost
                || (adjacent_spot_direction != direction_traveled && (potential_cost + 1) == spot_beyond_cost && !spot_beyond_wall && spot_beyond_cost != i32::MAX) {
                    let p = grid[working_spot_location.0][working_spot_location.1].path.clone();
                    grid[adjacent_spot_location.0][adjacent_spot_location.1].path.extend(p);
                    queue.push((Reverse(potential_cost), adjacent_spot_location));
                }
            }
        }
    }

    // for i in 0..grid.len() {
    //     let mut s = String::from("");
    //     for j in 0..grid[0].len() {
    //         if grid[end.0][end.1].path.contains(&(i, j)) {
    //             s.push('O');
    //         }
    //         else if grid[i][j].wall {
    //             s.push('#');
    //         }
    //         else {
    //             s.push('.');
    //         }
    //     }
    //     println!("{:?}", s);
    // }

    Ok(grid[end.0][end.1].path.len().to_string())
}
