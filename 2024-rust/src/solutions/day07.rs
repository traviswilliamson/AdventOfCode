use std::error::Error;

fn valid(target: i64, nums: Vec<i64>) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }
    let end = nums[nums.len() - 1];
    if target % end == 0 {
        if valid(target / end, nums[0..nums.len() - 1].to_vec()) {
            return true;
        }
    }
    valid(target - end, nums[0..nums.len() - 1].to_vec())
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n")
        .map(|l| {
            let mut stuff = l.split(":");
            let test_val = stuff.next().unwrap().parse::<i64>().unwrap();
            let nums: Vec<i64> = stuff.next().unwrap().split(" ").filter(|n| n.len() > 0).map(|n| n.parse::<i64>().unwrap()).collect();
            if valid(test_val, nums) { test_val }
            else { 0 }
        })
        .sum::<i64>()
        .to_string()
    )
}

fn valid_concat(target: i64, nums: Vec<i64>) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }
    let end = nums[nums.len() - 1];
    if target % end == 0 {
        if valid_concat(target / end, nums[0..nums.len() - 1].to_vec()) {
            return true;
        }
    }
    if target % 10_i64.pow(end.ilog10() + 1) == end {
        if valid_concat(target / 10_i64.pow(end.ilog10() + 1), nums[0..nums.len() - 1].to_vec()) {
            return true;
        }
    }
    valid_concat(target - end, nums[0..nums.len() - 1].to_vec())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n")
        .map(|l| {
            let mut stuff = l.split(":");
            let test_val = stuff.next().unwrap().parse::<i64>().unwrap();
            let nums: Vec<i64> = stuff.next().unwrap().split(" ").filter(|n| n.len() > 0).map(|n| n.parse::<i64>().unwrap()).collect();
            if valid_concat(test_val, nums) { test_val }
            else { 0 }
        })
        .sum::<i64>()
        .to_string()
    )
}
