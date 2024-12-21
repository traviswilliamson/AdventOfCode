use std::error::Error;
use memoize::memoize;

fn char_to_spot(c: &char, first: bool) -> (i8, i8) {
    match c {
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '^' => (0, 1),
        '>' => (1, 2),
        'A' => match first {
            true => (3, 2),
            false => (0, 2)
        },
        _ => panic!("Bad char parsing of {:?} at first: {:?}!", c, first)
    }
}

fn diff_to_path(diff: i8, vertical: bool) -> String {
    match (diff, vertical) {
        (d, true) if d < 0 => str::repeat("^", diff.abs() as usize),
        (d, true) if d > 0 => str::repeat("v", diff.abs() as usize),
        (d, false) if d < 0 => str::repeat("<", diff.abs() as usize),
        (d, false) if d > 0 => str::repeat(">", diff.abs() as usize),
        (0, _) => String::from(""),
        (_, _) => panic!("Diff of {:?} and vertical: {:?} not covered!", diff, vertical)
    }
}

#[memoize]
fn expand_path(path: String, depth: u16, first: bool) -> u64 {
    let mut current_spot = match first {
        true => (3, 2),
        false => (0, 2)
    };
    let bad_spot = match first {
        true => (3, 0),
        false => (0, 0)
    };
    let mut count: u64 = 0;
    for next_spot in path.chars().map(|c| char_to_spot(&c, first)) {
        let vert_diff = next_spot.0 - current_spot.0;
        let hor_diff = next_spot.1 - current_spot.1;
        let contains_bad = (current_spot.0 == bad_spot.0 || next_spot.0 == bad_spot.0) && (current_spot.1 == bad_spot.1 || next_spot.1 == bad_spot.1);
        let prefer_hor = (hor_diff < 0 && !contains_bad) || (hor_diff > 0 && contains_bad);

        // Build path to next spot
        let mut path = String::new();
        if prefer_hor {
            path.push_str(&diff_to_path(hor_diff, false));
            path.push_str(&diff_to_path(vert_diff, true));
        }
        else {
            path.push_str(&diff_to_path(vert_diff, true));
            path.push_str(&diff_to_path(hor_diff, false));
        }
        path.push('A');

        // Get length
        if depth == 0 {
            count += path.len() as u64;
        }
        else {
            count += expand_path(path, depth - 1, false);
        }
        // if first {
        //     println!("Found length from {:?} to {:?} using path: {:?}. Count so far: {:?}", current_spot, next_spot, path, count);
        //     println!("Vert diff {:?}, hor diff {:?}, contains bad {:?}, prefer hor {:?}", vert_diff, hor_diff, contains_bad, prefer_hor);
        // }
        current_spot = next_spot;
    }
    // println!("{:?}", count);
    count
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n").map(|l| {
        expand_path(l.to_string(), 2, true) * l[0..l.len() - 1].parse::<u64>().unwrap()
    }).sum::<u64>().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n").map(|l| {
        expand_path(l.to_string(), 25, true) * l[0..l.len() - 1].parse::<u64>().unwrap()
    }).sum::<u64>().to_string())
}
