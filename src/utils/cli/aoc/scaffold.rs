use std::{
    fs::{create_dir, File, OpenOptions},
    io::{Error, Write},
};

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

const DAY_TEMPLATE: &str = r#"pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> i32 {
    i32::MAX
}

pub fn part_two(input: &str) -> i32 {
    i32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}

"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

pub fn scaffold_day(year: u16, day: u8) -> Result<(), Error> {
    let day_file_path = format!("src/advent_of_code/aoc{year}/day{day:02}.rs");

    let mut file = match safe_create_file(&day_file_path) {
        Ok(file) => file,
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create day file",
            ))
        }
    };

    match file.write_all(DAY_TEMPLATE.as_bytes()) {
        Ok(()) => Ok(()),
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write template",
            ))
        }
    }
}
