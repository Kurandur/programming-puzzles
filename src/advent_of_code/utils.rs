use std::fs;

pub fn read_input(year: u16, day: u8) -> String {
    let path = format!("src/advent_of_code/aoc{}/input/{:02}.txt", year, day);
    fs::read_to_string(path.clone()).expect(&format!("Unable to read input from: {}", path))
}
