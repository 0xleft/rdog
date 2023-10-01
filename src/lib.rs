use reqwest::{self};
use std::{fs, io::Write};

pub fn download_file(host: &str, path: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}{}", host, path);
    let content = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}