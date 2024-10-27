use std::{error::Error, fs::File, io::{self, Read, Write}, path::{Path, PathBuf}};

use async_std::fs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Executing day 1");

    let download_path = setup_input_file(1).await?;
    Ok(())
}

async fn setup_input_file(day : u8) -> Result<PathBuf, Box<dyn Error>> {
    let mut file_path = std::env::current_dir()?;
    file_path.push("input");
    if !Path::exists(&file_path.as_path()) {
        fs::create_dir(&file_path).await?;
    }
    file_path.push(format!("{day}.txt"));

    if Path::exists(&file_path) {
        println!("{} already exists", file_path.display());
        return Ok(file_path);
    }
    let mut input_file_handle = File::create_new(&file_path)?;
    
    let input_data = download_input_file(day).await?;

    println!("Writing input data to file");
    input_file_handle.write(input_data.as_bytes())?;

    Ok(file_path)
}

async fn download_input_file(day : u8) -> Result<String, Box<dyn Error>>  {
    println!("Downloading input data");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header(reqwest::header::COOKIE, read_cookie_file()?)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

fn read_cookie_file() -> Result<String, Box<dyn Error>> {
    println!("Reading cookie file");
    let mut file_handle = File::open("input/cookie.txt")?;
    let mut cookie = String::new();
    file_handle.read_to_string(&mut cookie)?;

    if cookie.contains('\n') {
        return Err(Box::<dyn Error>::from("Cookie file has more than one line"));
    }

    Ok(cookie)
}