use std::{collections::HashMap, error::Error};

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut stones: Vec<i64> = _input.trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
    for _ in 0..25 {
        for i in 0..stones.len() {
            let n = stones[i];
            if n == 0 {
                stones[i] = 1;
            }
            // Even digits
            else if n.ilog10() % 2 == 1 {
                stones.push(n % (10_i64.pow((n.ilog10() + 1) / 2)));
                stones[i] = n / (10_i64.pow((n.ilog10() + 1) / 2));
            }
            else {
                stones[i] = stones[i] * 2024;
            }
        }
    }
    Ok(stones.len().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut stones: HashMap<i64, i64> = _input.trim().split(" ").map(|s| s.parse::<i64>().unwrap()).fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });
    for _ in 0..75 {
        let mut temp_stones: HashMap<i64, i64> = HashMap::new(); 
        for (num, count) in stones {
            if num == 0 {
                *temp_stones.entry(1).or_insert(0) += count;
            }
            // Even digits
            else if num.ilog10() % 2 == 1 {
                *temp_stones.entry(num % (10_i64.pow((num.ilog10() + 1) / 2))).or_insert(0) += count;
                *temp_stones.entry(num / (10_i64.pow((num.ilog10() + 1) / 2))).or_insert(0) += count;
            }
            else {
                *temp_stones.entry(num * 2024).or_insert(0) += count;
            }
        }
        stones = temp_stones;
    }
    Ok(stones.into_values().sum::<i64>().to_string())
}
