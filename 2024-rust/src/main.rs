use std::{error::Error, time::SystemTime};

mod file_io;
mod server_io;
mod solutions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (day, first) = file_io::read_day_file()?;
    file_io::setup_input_file(day).await?;

    let input_data = file_io::read_input_file(day)?;

    println!("Executing day {day}, first: {first}");

    let day_functions: Vec<Vec<fn(String) -> Result<String, Box<dyn Error>>>> =
        vec![
            vec![solutions::day01_first, solutions::day01_second],
            vec![solutions::day02_first, solutions::day02_second],
            vec![solutions::day03_first, solutions::day03_second],
            vec![solutions::day04_first, solutions::day04_second],
            vec![solutions::day05_first, solutions::day05_second],
            vec![solutions::day06_first, solutions::day06_second],
            vec![solutions::day07_first, solutions::day07_second],
            vec![solutions::day08_first, solutions::day08_second],
            vec![solutions::day09_first, solutions::day09_second],
            vec![solutions::day10_first, solutions::day10_second],
            vec![solutions::day11_first, solutions::day11_second],
            vec![solutions::day12_first, solutions::day12_second],
            vec![solutions::day13_first, solutions::day13_second],
            vec![solutions::day14_first, solutions::day14_second],
            vec![solutions::day15_first, solutions::day15_second],
            vec![solutions::day16_first, solutions::day16_second],
            vec![solutions::day17_first, solutions::day17_second],
            vec![solutions::day18_first, solutions::day18_second],
            vec![solutions::day19_first, solutions::day19_second],
            vec![solutions::day20_first, solutions::day20_second],
            vec![solutions::day21_first, solutions::day21_second],
            vec![solutions::day22_first, solutions::day22_second],
            vec![solutions::day23_first, solutions::day23_second],
            vec![solutions::day24_first, solutions::day24_second],
            vec![solutions::day25_first, solutions::day25_second]
        ];
    let now = SystemTime::now();
    let answer = day_functions[day as usize - 1][match first {
        true => 0,
        false => 1,
    }](input_data)?;
    let elapsed = now.elapsed()?.as_micros();
    println!("Finished computing. Answer: {answer}");
    println!("Execution took: {elapsed} µs");

    Ok(())
}
