use scraper::{Html, Selector};
use std::{
    fs::{self, create_dir, File, OpenOptions},
    io::{Error, Write},
};

use super::SessionManager;

fn create_path(path: &str) -> Result<(), std::io::Error> {
    create_dir(path)
}

pub fn scaffold_year(year: u16) -> Result<(), std::io::Error> {
    println!("Creating aoc folder for the year {}", year);

    let root_path = format!("src/advent_of_code/aoc{}", year);
    let input_path = format!("{}/input", root_path);
    let input_gitkeep = format!("{}/.gitkeep", input_path);
    let mod_rs_path = format!("{}/mod.rs", root_path);

    match create_path(&root_path) {
        Ok(_) => {
            println!("Created folder {}", &root_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while creating year folder",
            ))
        }
    }

    match create_path(&input_path) {
        Ok(_) => {
            println!("Created folder {}", &input_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while creating input folder",
            ))
        }
    }

    match safe_create_file(&input_gitkeep) {
        Ok(_) => {
            println!("Created input folder .gitkeep under {}", &input_gitkeep);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to mod.rs",
            ))
        }
    };

    match safe_create_file(&mod_rs_path) {
        Ok(_) => {
            println!("Created mod.rs under {}", &mod_rs_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to mod.rs",
            ))
        }
    };

    Ok(())
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn download_puzzle_text(year: u16, day: u8) -> Result<String, Error> {
    let token = SessionManager::new()
        .get_session_token()
        .expect("No session token set.");
    let base_url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let client = reqwest::blocking::Client::new();
    let mut request_builder = client.get(base_url.clone());
    request_builder = request_builder.header("Cookie", format!("session={}", token));
    let request = match request_builder.build() {
        Ok(r) => r,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Request error",
            ))
        }
    };
    let response = match client.execute(request) {
        Ok(r) => r,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Response error",
            ))
        }
    };
    let body = match response.text() {
        Ok(body) => body,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Body error")),
    };

    let document = Html::parse_document(&body);

    let mut markdown_buffer = String::new();
    let article_selector = match Selector::parse("article") {
        Ok(selector) => selector,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Selector parse error: {}", e),
            ))
        }
    };
    for article in document.select(&article_selector) {
        markdown_buffer.push_str(html2md::parse_html(article.inner_html().as_str()).as_str());
        markdown_buffer.push_str("\n\n\n");
    }

    Ok(markdown_buffer)
}

pub fn scaffold_day(year: u16, day: u8) -> Result<(), Error> {
    let day_folder_path = format!("src/advent_of_code/aoc{year}/day{day:02}");
    let day_file_path = format!("src/advent_of_code/aoc{year}/day{day:02}/day{day:02}.rs");
    let day_mod_file_path = format!("src/advent_of_code/aoc{year}/day{day:02}/mod.rs");
    let day_readme_file_path = format!("src/advent_of_code/aoc{year}/day{day:02}/README.md");

    // create folder
    match create_path(&day_folder_path) {
        Ok(_) => {
            println!("Created folder {}", &day_folder_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while creating day folder",
            ))
        }
    };

    // create day file
    let mut day_file = match safe_create_file(&day_file_path) {
        Ok(file) => {
            println!("Created file {}", &day_file_path);
            file
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create day file",
            ))
        }
    };

    let day_file_template =
        fs::read_to_string("src/utils/cli/aoc/templates/day_file_template.txt")?;
    match day_file.write_all(day_file_template.as_bytes()) {
        Ok(()) => {
            println!("Wrote day template to file {}", &day_file_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write day mod.rs template",
            ))
        }
    };

    // day mod.rs
    let mut day_mod_file = match safe_create_file(&day_mod_file_path) {
        Ok(file) => file,
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create day file",
            ))
        }
    };
    let day_mod_file_template =
        fs::read_to_string("src/utils/cli/aoc/templates/mod_file_template.txt")?;
    let filled_day_mod_file_template =
        day_mod_file_template.replace("{{DAY}}", &format!("{:02}", day));

    match day_mod_file.write_all(filled_day_mod_file_template.as_bytes()) {
        Ok(()) => {
            println!("Wrote mod template to file {}", &day_mod_file_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write day mod.rs template",
            ))
        }
    };

    // Readme
    let mut day_readme_file = match safe_create_file(&day_readme_file_path) {
        Ok(file) => file,
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create day readme",
            ))
        }
    };
    let day_readme_template = fs::read_to_string("src/utils/cli/aoc/templates/day_readme.txt")?;

    let mut filled_day_readme_template = day_readme_template
        .replace("{{DAY}}", &format!("{:02}", day))
        .replace("{{YEAR}}", &format!("{}", year));

    if let Ok(text) = download_puzzle_text(year, day) {
        filled_day_readme_template = filled_day_readme_template.replace("{{PUZZLE_TEXT}}", &text);
    }

    match day_readme_file.write_all(filled_day_readme_template.as_bytes()) {
        Ok(()) => {
            println!("Wrote mod template to file {}", &day_mod_file_path);
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write day mod.rs template",
            ))
        }
    };

    Ok(())
}
