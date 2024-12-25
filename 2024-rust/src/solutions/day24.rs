use std::{collections::{HashMap, VecDeque}, error::Error};

use itertools::Itertools;

struct Gate<'a> {
    i1: &'a str,
    i2: &'a str,
    op: &'a str,
    out: &'a str
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut halves = _input.split("\n\n");
    let mut states = halves.next().unwrap().split_terminator("\n").map(|l| {
        let mut s = l.split(": ");
        (s.next().unwrap(), s.next().unwrap() != "0")
    }).collect::<HashMap<&str, bool>>();
    let mut gates = halves.next().unwrap().split_terminator("\n").map(|l| {
        let mut s = l.split(" ");
        Gate {
            i1: s.next().unwrap(),
            op: s.next().unwrap(),
            i2: s.next().unwrap(),
            out: s.skip(1).next().unwrap()
        }
    }).collect::<VecDeque<Gate>>();
    
    // Run gates
    while !gates.is_empty() {
        let g = gates.pop_front().unwrap();
        if states.contains_key(g.i1) && states.contains_key(g.i2) {
            let i1v = states.get(g.i1).unwrap();
            let i2v = states.get(g.i2).unwrap();
            let out = match g.op {
                "AND" => *i1v && *i2v,
                "OR" => *i1v || *i2v,
                "XOR" => *i1v ^ *i2v,
                _ => panic!("Bad op of {:?}!", g.op)
            };
            states.insert(g.out, out);
        }
        else {
            gates.push_back(g);
        }
    }

    // for (k, v) in states.iter().sorted() {
    //     println!("{:?} {:?}", k, *v);
    // }

    // Build output from 'z's
    let mut output = 0_u64;
    for (_, v) in states.iter().filter(|(k, _)| {
        k.starts_with('z')
    }).sorted().rev() {
        output <<= 1;
        output += *v as u64;
        // println!("Output: {:#b}, v {:?}", output, *v);
    }
    Ok(output.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    return Err(Box::<dyn Error>::from("Not implemented!"));
}
