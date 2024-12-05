use std::{cmp::Ordering, collections::HashSet, error::Error};

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut data = _input.split("\n\n");
    let rules: HashSet<(i32, i32)> = data.next().unwrap()
        .split("\n")
        .map(|l| {
            let mut nums = l.split("|");
            (nums.next().unwrap().parse::<i32>().unwrap(),
                nums.next().unwrap().parse::<i32>().unwrap())
    }).collect();
    let stacks: Vec<Vec<i32>> = data.next().unwrap()
        .split("\n").map(|l| {
            l.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
        }).collect();

    let good_sum: i32 = stacks
        .iter()
        .filter(|s| s.is_sorted_by(|a, b| rules.contains(&(*a,*b))))
        .map(|s| s[s.len()/2])
        .sum();
    Ok(good_sum.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut data = _input.split("\n\n");
    let rules: HashSet<(i32, i32)> = data.next().unwrap()
        .split("\n")
        .map(|l| {
            let mut nums = l.split("|");
            (nums.next().unwrap().parse::<i32>().unwrap(),
                nums.next().unwrap().parse::<i32>().unwrap())
    }).collect();
    let mut stacks: Vec<Vec<i32>> = data.next().unwrap()
        .split("\n").map(|l| {
            l.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
        }).collect();

    let good_sum: i32 = stacks
        .iter_mut()
        .filter(|s| !s.is_sorted_by(|a, b|
            rules.contains(&(*a,*b))))
        .map(|s: &mut Vec<i32>| {
            s.sort_by(|a, b| {
            if rules.contains(&(*a,*b)) {
                Ordering::Less
            }
            else {
                Ordering::Greater
            }});
            s[s.len()/2]
        })
        .sum();
    Ok(good_sum.to_string())
}
