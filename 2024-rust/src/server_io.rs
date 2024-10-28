use crate::file_io::read_cookie_file;
use std::{collections::HashMap, error::Error};

pub async fn download_input_file(day: u8) -> Result<String, Box<dyn Error>> {
    println!("Downloading input data");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header(reqwest::header::COOKIE, read_cookie_file()?)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
pub async fn upload_answer(day: u8, first: bool, answer: String) -> Result<String, Box<dyn Error>> {
    println!("Uploading solution to server");

    let json_data = HashMap::from([
        ("level", match first {true => "1", false => "2"}),
        ("answer", &answer)
    ]);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://adventofcode.com/2024/day/{day}/answer"))
        .header(reqwest::header::COOKIE, read_cookie_file()?)
        .json(&json_data)
        .send()
        .await?
        .text()
        .await?;

    // TODO: Parse response
    println!("{response}");

    Ok(response)
}