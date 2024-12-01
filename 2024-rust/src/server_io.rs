use crate::file_io::read_cookie_file;
use std::error::Error;

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