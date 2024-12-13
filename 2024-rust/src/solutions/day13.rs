use std::error::Error;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n\n").fold(0, |mut count, problem| {
        let mut lines = problem.split("\n");
        let a_coefficients: Vec<i32> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i32>().unwrap()).collect();
        let b_coefficients: Vec<i32> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i32>().unwrap()).collect();
        let prize_coefficients: Vec<i32> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i32>().unwrap()).collect();

        let B = ((a_coefficients[1] * prize_coefficients[0]) - (a_coefficients[0] * prize_coefficients[1])) / ((a_coefficients[1] * b_coefficients[0]) - (a_coefficients[0] * b_coefficients[1]));
        let A = (prize_coefficients[0] - (b_coefficients[0] * B)) / a_coefficients[0];
        if (prize_coefficients[0] == (a_coefficients[0] * A) + (b_coefficients[0] * B))
            && (prize_coefficients[1] == (a_coefficients[1] * A) + (b_coefficients[1] * B)) {
                count += (A * 3) + B;
            }
        count
    }).to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    Ok(_input.split_terminator("\n\n").fold(0, |mut count, problem| {
        let mut lines = problem.split("\n");
        let a_coefficients: Vec<i64> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i64>().unwrap()).collect();
        let b_coefficients: Vec<i64> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i64>().unwrap()).collect();
        let prize_coefficients: Vec<i64> = lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|s| s[2..].parse::<i64>().unwrap() + 10000000000000).collect();

        let B = ((a_coefficients[1] * prize_coefficients[0]) - (a_coefficients[0] * prize_coefficients[1])) / ((a_coefficients[1] * b_coefficients[0]) - (a_coefficients[0] * b_coefficients[1]));
        let A = (prize_coefficients[0] - (b_coefficients[0] * B)) / a_coefficients[0];
        if (prize_coefficients[0] == (a_coefficients[0] * A) + (b_coefficients[0] * B))
            && (prize_coefficients[1] == (a_coefficients[1] * A) + (b_coefficients[1] * B)) {
                count += (A * 3) + B;
            }
        count
    }).to_string())
}
