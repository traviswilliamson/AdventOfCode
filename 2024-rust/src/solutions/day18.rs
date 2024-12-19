use std::{cmp::Reverse, collections::BinaryHeap, error::Error, usize};

const LENGTH: i32 = 71;
const LENGTH_U: usize = 71;
const HEIGHT: i32 = 71;
const HEIGHT_U: usize = 71;

fn in_bounds(spot: (i32, i32)) -> bool {
    spot.0 >= 0 && spot.0 < HEIGHT && spot.1 >= 0 && spot.1 < LENGTH
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid = [[i32::MAX; LENGTH_U]; HEIGHT_U];
    _input.split_terminator("\n").take(1024).for_each(|l| {
        let mut i = l.split(",");
        grid[i.next().unwrap().parse::<usize>().unwrap()][i.next().unwrap().parse::<usize>().unwrap()] = i32::MIN;
    });
    let mut queue = BinaryHeap::from([(Reverse(0 + LENGTH + HEIGHT), (0 as usize, 0 as usize))]);
    grid[0][0] = 0;
    'outer: loop {
        let here = queue.pop().unwrap().1;
        let here_spot = grid[here.0][here.1];
        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let adj = ((here.0 as i32) + d.0, (here.1 as i32) + d.1);
            if in_bounds(adj) {
                let adj = (adj.0 as usize, adj.1 as usize);
                let adj_spot = grid[adj.0][adj.1];
                if here_spot + 1 < adj_spot {
                    grid[adj.0][adj.1] = here_spot + 1;
                    if adj == (LENGTH_U - 1, HEIGHT_U - 1) {
                        break 'outer;
                    }
                    // A*
                    queue.push((Reverse(here_spot + 1 + (LENGTH - (adj.0 as i32)) + (HEIGHT - (adj.1 as i32))), adj));
                }
            }
        }
    }
    Ok(grid[LENGTH_U - 1][HEIGHT_U - 1].to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut grid = [[(i32::MAX, -1); LENGTH_U]; HEIGHT_U];
    let mut bytes = vec![];
    _input.split_terminator("\n").enumerate().for_each(|(t, l)| {
        let mut iter = l.split(",");
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        grid[y][x] = (i32::MIN, t as i32);
        bytes.push((y, x));
    });
    let mut time = bytes.len() - 1;
    let mut queue = BinaryHeap::from([(Reverse(0 + LENGTH + HEIGHT), (0 as usize, 0 as usize))]);
    let mut iterations = 0;
    grid[0][0] = (0, -1);
    'outer: loop {
        let mut here_o = queue.pop();
        if here_o.is_none() {
            // Ran out? Reduce time, start from there
            time -= 1;
            iterations += 1;
            here_o = Some((Reverse(0), bytes[time]));
            // println!("Time: {:?}, iterations: {:?}, new spot {:?}", time, iterations, (bytes[time]));
            // Is this adjacent to a place we've been?
            let mut adjacent = false;
            for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let adj = ((bytes[time].0 as i32) + d.0, (bytes[time].1 as i32) + d.1);
                if in_bounds(adj) {
                    let adj = (adj.0 as usize, adj.1 as usize);
                    let adj_spot = grid[adj.0][adj.1].0;
                    if adj_spot != i32::MAX && adj_spot != i32::MIN {
                        adjacent = true;
                        break;
                    }
                }
            }
            if !adjacent {
                continue;
            }
            grid[bytes[time].0][bytes[time].1].0 = iterations;
        }
        let here = here_o.unwrap().1;
        let here_spot = grid[here.0][here.1].0;
        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let adj = ((here.0 as i32) + d.0, (here.1 as i32) + d.1);
            if in_bounds(adj) {
                let adj = (adj.0 as usize, adj.1 as usize);
                let adj_spot = grid[adj.0][adj.1];
                if here_spot + 1 < adj_spot.0 || ((adj_spot.0 == i32::MIN) && adj_spot.1 > (time as i32)) {
                    grid[adj.0][adj.1].0 = here_spot + 1;
                    if adj == (LENGTH_U - 1, HEIGHT_U - 1) {
                        break 'outer;
                    }
                    // A*
                    queue.push((Reverse(here_spot + 1 + (LENGTH - (adj.0 as i32)) + (HEIGHT - (adj.1 as i32))), adj));
                    iterations += 1;
                }
            }
        }
    }
    Ok(format!("{:?}", (bytes[time].1, bytes[time].0)))
}
