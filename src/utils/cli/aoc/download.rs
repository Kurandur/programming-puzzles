use std::io::Write;
use std::path::Path;
use std::{fs::File, io::Read};

use reqwest::header::COOKIE;

use super::SessionManager;

pub fn download_input(year: u16, day: u8) -> Result<(), std::io::Error> {
    let token = SessionManager::new()
        .get_session_token()
        .expect("No session token set.");
    let formatted_token = format!("session={}", token);

    let client = reqwest::blocking::Client::new();
    let request_url = format!("src/advent_of_code/aoc{year}/input/{day:02}.txt");
    let input_file_path = Path::new(&request_url);

    if input_file_path.exists() {
        return Ok(());
    }

    let request_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let mut response = client
        .get(request_url)
        .header(COOKIE, formatted_token)
        .send()
        .unwrap();

    let mut body = String::new();
    response.read_to_string(&mut body)?;

    let mut file = File::create(input_file_path)?;
    file.write_all(body.as_bytes())?;

    Ok(())
}
