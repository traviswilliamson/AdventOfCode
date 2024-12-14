use std::{collections::HashSet, error::Error};

const XMAX: i32 = 101;
const YMAX: i32 = 103;
const XMID: i32 = 50;
const YMID: i32 = 51;
const ITERATIONS: i32 = 100;

struct Bot {
    start_x: i32,
    start_y: i32,
    v_x: i32,
    v_y: i32
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut quads = vec![0, 0, 0, 0];
    _input.split_terminator("\n").map(|line| {
        let mut b = line.split(" ");
        let mut start = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
        let mut vel = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
        Bot {
            start_x: start.next().unwrap().parse().unwrap(),
            start_y: start.next().unwrap().parse().unwrap(),
            v_x: vel.next().unwrap().parse().unwrap(),
            v_y: vel.next().unwrap().parse().unwrap()
        }
    }).for_each(|b| {
        let ending_x = (((b.start_x + (ITERATIONS * b.v_x)) % XMAX) + XMAX) % XMAX;
        let ending_y = (((b.start_y + (ITERATIONS * b.v_y)) % YMAX) + YMAX) % YMAX;
        if ending_x != XMID && ending_y != YMID {
            quads[(((ending_x - 1) / XMID) + ((ending_y - 1) / YMID * 2)) as usize] += 1;
        }
    });
    Ok(quads.iter().fold(1, |prod, quad| prod * quad ).to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut bots: Vec<Bot> = _input.split_terminator("\n").map(|line| {
        let mut b = line.split(" ");
        let mut start = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
        let mut vel = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
        Bot {
            start_x: start.next().unwrap().parse().unwrap(),
            start_y: start.next().unwrap().parse().unwrap(),
            v_x: vel.next().unwrap().parse().unwrap(),
            v_y: vel.next().unwrap().parse().unwrap()
        }
    }).collect();
    

    let mut iteration = 0;
    let mut found = 0;

    loop {
        let mut doubled = false;
        let mut set = HashSet::new();
        for bot in bots.iter() {
            if set.contains(&(bot.start_x + bot.start_y * YMAX)) {
                doubled = true;
                break;
            }
            set.insert(bot.start_x + bot.start_y * YMAX);
        }
        if !doubled {
            found += 1;
            if found > 1 { break; }
        }

        
        bots.iter_mut().for_each(|b| {
            b.start_x = (((b.start_x + b.v_x) % XMAX) + XMAX) % XMAX;
            b.start_y = (((b.start_y + b.v_y) % YMAX) + YMAX) % YMAX;
        });
        iteration += 1;
    }

    Ok(iteration.to_string())
}
