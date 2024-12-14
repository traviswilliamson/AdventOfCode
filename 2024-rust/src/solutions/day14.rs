use std::error::Error;

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

// Unique locations method
// pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
//     let mut bots: Vec<Bot> = _input.split_terminator("\n").map(|line| {
//         let mut b = line.split(" ");
//         let mut start = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
//         let mut vel = b.next().unwrap().split("=").skip(1).next().unwrap().split(",");
//         Bot {
//             start_x: start.next().unwrap().parse().unwrap(),
//             start_y: start.next().unwrap().parse().unwrap(),
//             v_x: vel.next().unwrap().parse().unwrap(),
//             v_y: vel.next().unwrap().parse().unwrap()
//         }
//     }).collect();
    

//     let mut iteration = 0;
//     let mut found = 0;

//     loop {
//         let mut doubled = false;
//         let mut set = HashSet::new();
//         for bot in bots.iter() {
//             if set.contains(&(bot.start_x + bot.start_y * YMAX)) {
//                 doubled = true;
//                 break;
//             }
//             set.insert(bot.start_x + bot.start_y * YMAX);
//         }
//         if !doubled {
//             found += 1;
//             if found > 1 { break; }
//         }

        
//         bots.iter_mut().for_each(|b| {
//             b.start_x = (((b.start_x + b.v_x) % XMAX) + XMAX) % XMAX;
//             b.start_y = (((b.start_y + b.v_y) % YMAX) + YMAX) % YMAX;
//         });
//         iteration += 1;
//     }

//     Ok(iteration.to_string())
// }

// Chinese remainder theorem method
fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

fn euler_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = euler_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn modulus_invert(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = euler_gcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i32], modulii: &[i32]) -> Option<i32> {
    let prod = modulii.iter().product::<i32>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * modulus_invert(p, modulus)? * p
    }

    Some(sum % prod)
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
    
    let mut x_best = (0, f32::MAX);
    let mut y_best = (0, f32::MAX);
    for i in 0..YMAX {
        if i < XMAX {
            let x_std_dev = std_deviation(&bots.iter().map(|b| b.start_x).collect::<Vec<i32>>());
            if x_std_dev.unwrap() < x_best.1 {
                x_best = (i, x_std_dev.unwrap())
            }
        }
        let y_std_dev = std_deviation(&bots.iter().map(|b| b.start_y).collect::<Vec<i32>>());
        if y_std_dev.unwrap() < y_best.1 {
            y_best = (i, y_std_dev.unwrap())
        }

        // step
        bots.iter_mut().for_each(|b| {
            b.start_x = (((b.start_x + b.v_x) % XMAX) + XMAX) % XMAX;
            b.start_y = (((b.start_y + b.v_y) % YMAX) + YMAX) % YMAX;
        });
    }

    Ok(chinese_remainder(&[x_best.0, y_best.0], &[XMAX, YMAX]).unwrap().to_string())
}