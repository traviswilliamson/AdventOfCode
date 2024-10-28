use async_std::fs;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub async fn setup_input_file(day: u8) -> Result<(), Box<dyn Error>> {
    println!("Setting up day {day}");
    let mut file_path = std::env::current_dir()?;
    file_path.push("input");
    if !Path::exists(&file_path.as_path()) {
        fs::create_dir(&file_path).await?;
    }
    file_path.push(format!("{day}.txt"));

    if Path::exists(&file_path) {
        println!("{} already exists", file_path.display());
        return Ok(());
    }
    let mut input_file_handle = File::create_new(&file_path)?;

    let input_data = crate::server_io::download_input_file(day).await?;

    println!("Writing input data to file");
    input_file_handle.write(input_data.as_bytes())?;

    Ok(())
}

pub fn read_cookie_file() -> Result<String, Box<dyn Error>> {
    println!("Reading cookie file");
    let mut file_handle = File::open("input/cookie.txt")?;
    let mut cookie = String::new();
    file_handle.read_to_string(&mut cookie)?;

    if cookie.contains('\n') {
        return Err(Box::<dyn Error>::from("Cookie file has more than one line"));
    }

    Ok(cookie)
}

pub fn read_day_file() -> Result<(u8, bool), Box<dyn Error>> {
    println!("Reading day file");
    let mut file_handle = File::open("input/day.txt")?;
    let mut buffer = String::new();
    file_handle.read_to_string(&mut buffer)?;
    let day = buffer[..2].parse::<u8>()?;
    let first = buffer[2..3].parse::<char>()? == 'a';

    Ok((day, first))
}

pub fn read_input_file(day: u8) -> Result<String, Box<dyn Error>> {
    println!("Reading input file for day {day}");
    let mut file_handle = File::open(format!("input/{day}.txt"))?;
    let mut data = String::new();
    file_handle.read_to_string(&mut data)?;

    Ok(data)
}
