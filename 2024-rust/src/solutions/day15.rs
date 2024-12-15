use std::error::Error;
use phf::phf_map;

const WALL: char = '#';
const OPEN: char = '.';
const BOT: char = '@';
const BOX: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
static MOVES: phf::Map<char, (i32, i32)> = phf_map! {
    '>' => (0, 1),
    'v' => (1, 0),
    '<' => (0, -1),
    '^' => (-1, 0)
};

static mut HEIGHT: i32 = 0;
static mut LENGTH: i32 = 0;

fn in_bounds(spot: (i32, i32)) -> bool {
    unsafe {
        spot.0 > 0 && spot.0 < HEIGHT && spot.1 > 0 && spot.1 < LENGTH
    }
}

fn execute_move(grid: &mut Vec<Vec<char>>, spot: (usize, usize), order: (i32, i32)) -> (bool, (usize, usize)) {
    let next_spot = ((spot.0 as i32) + order.0, (spot.1 as i32) + order.1);
    if in_bounds(next_spot) {
        let next_spot = (next_spot.0 as usize, next_spot.1 as usize);
        match grid[next_spot.0][next_spot.1] {
            WALL => (false, spot),
            OPEN => {
                grid[next_spot.0][next_spot.1] = grid[spot.0][spot.1];
                grid[spot.0][spot.1] = OPEN;
                (true, next_spot)
            },
            BOX => {
                if execute_move(grid, next_spot, order).0 {
                    grid[next_spot.0][next_spot.1] = grid[spot.0][spot.1];
                    grid[spot.0][spot.1] = OPEN;
                    (true, next_spot)
                }
                else {
                    (false, spot)
                }
            },
            _ => panic!("BAD CASE, {:?}", grid[next_spot.0][next_spot.1])
        }
    }
    else {
        (false, spot)
    }
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut sections = _input.split("\n\n");
    let mut grid: Vec<Vec<char>> = sections.next().unwrap()
        .split_terminator("\n").map(|line| {
            line.chars().collect()
    }).collect();
    unsafe {
        HEIGHT = grid.len() as i32;
        LENGTH = grid[0].len() as i32;
    }
    let orders = sections.next().unwrap()
        .chars().filter(|c| *c != '\n');

    let mut bot_spot = (grid.iter().position(|l| l.contains(&BOT)).unwrap(), 0);
    bot_spot.1 = grid[bot_spot.0].iter().position(|c| *c == BOT).unwrap();
    for order in orders {
        bot_spot = execute_move(&mut grid, bot_spot, *MOVES.get(&order).unwrap()).1;
    }

    let mut score = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == BOX {
                score += (100 * i) + j;
            }
        }
    }
    Ok(score.to_string())
}

fn grid_move(grid: &mut Vec<Vec<char>>, new_spot: (usize, usize), old_spot: (usize, usize)) {
    grid[new_spot.0][new_spot.1] = grid[old_spot.0][old_spot.1];
    grid[old_spot.0][old_spot.1] = OPEN;
}

fn execute_checked_move(grid: &mut Vec<Vec<char>>, spot: (usize, usize), order: (i32, i32), confirmed: bool) -> (bool, (usize, usize)) {
    let next_spot = ((spot.0 as i32) + order.0, (spot.1 as i32) + order.1);
    if in_bounds(next_spot) {
        let next_spot = (next_spot.0 as usize, next_spot.1 as usize);
        match grid[next_spot.0][next_spot.1] {
            WALL => (false, spot),
            OPEN => {
                if confirmed {
                    grid_move(grid, next_spot, spot);
                }
                (true, next_spot)
            },
            BOX_LEFT => {
                if execute_checked_move(grid, next_spot, order, confirmed).0
                && (order.0 == 0 || execute_checked_move(grid, (next_spot.0, next_spot.1 + 1), order, confirmed).0) {
                    if confirmed {
                        grid_move(grid, next_spot, spot);
                    }
                    (true, next_spot)
                }
                else {
                    (false, spot)
                }
            },
            BOX_RIGHT => {
                if execute_checked_move(grid, next_spot, order, confirmed).0
                && (order.0 == 0 || execute_checked_move(grid, (next_spot.0, next_spot.1 - 1), order, confirmed).0) {
                    if confirmed {
                        grid_move(grid, next_spot, spot);
                    }
                    (true, next_spot)
                }
                else {
                    (false, spot)
                }
            },
            _ => panic!("BAD CASE, {:?}", grid[next_spot.0][next_spot.1])
        }
    }
    else {
        (false, spot)
    }
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut sections = _input.split("\n\n");
    let mut grid: Vec<Vec<char>> = sections.next().unwrap()
        .split_terminator("\n").map(|line| {
            line.chars().fold(Vec::with_capacity(line.len() * 2), |mut s: Vec<char>, c| {
                match c {
                    WALL => {
                        s.push(WALL);
                        s.push(WALL);
                    },
                    OPEN => {
                        s.push(OPEN);
                        s.push(OPEN);
                    }
                    BOT => {
                        s.push(BOT);
                        s.push(OPEN);
                    }
                    BOX => {
                        s.push(BOX_LEFT);
                        s.push(BOX_RIGHT);
                    }
                    _ => panic!("BAD INPUT!")
                }
                s
            })
    }).collect();
    unsafe {
        HEIGHT = grid.len() as i32;
        LENGTH = grid[0].len() as i32;
    }
    let orders = sections.next().unwrap()
        .chars().filter(|c| *c != '\n');

    let mut bot_spot = (grid.iter().position(|l| l.contains(&BOT)).unwrap(), 0);
    bot_spot.1 = grid[bot_spot.0].iter().position(|c| *c == BOT).unwrap();
    for order in orders {
        if execute_checked_move(&mut grid, bot_spot, *MOVES.get(&order).unwrap(), false).0 {
            bot_spot = execute_checked_move(&mut grid, bot_spot, *MOVES.get(&order).unwrap(), true).1;
        }
    }

    let mut score = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == BOX_LEFT {
                score += (100 * i) + j;
            }
        }
    }
    Ok(score.to_string())
}
