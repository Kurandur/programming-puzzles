use self::utils::read_input;

mod aoc2023;
mod aoc2024;
mod utils;

pub fn run_solution(year: u16, day: u8, part: u8) -> Result<String, std::io::Error> {
    let input = &read_input(year, day);

    let result = match (year, day, part) {
        (2023, 1, 1) => {
            SolutionResult::U32(aoc2023::day01::part_one(&aoc2023::day01::generator(input)))
        }
        (2023, 1, 2) => {
            SolutionResult::U32(aoc2023::day01::part_two(&aoc2023::day01::generator(input)))
        }
        (2023, 2, 1) => {
            SolutionResult::U32(aoc2023::day02::part_one(aoc2023::day02::generator(input)))
        }
        (2023, 2, 2) => {
            SolutionResult::U32(aoc2023::day02::part_two(aoc2023::day02::generator(input)))
        }
        (2023, 3, 1) => {
            SolutionResult::U32(aoc2023::day03::part_one(&aoc2023::day03::generator(input)))
        }
        (2023, 3, 2) => {
            SolutionResult::U32(aoc2023::day03::part_two(&aoc2023::day03::generator(input)))
        }
        (2023, 4, 1) => {
            SolutionResult::U32(aoc2023::day04::part_one(aoc2023::day04::generator(input)))
        }
        (2023, 4, 2) => {
            SolutionResult::U32(aoc2023::day04::part_two(&aoc2023::day04::generator(input)))
        }
        (2024, 1, 1) => {
            SolutionResult::U32(aoc2024::day01::part_one(&aoc2024::day01::generator(input)))
        }
        (2024, 1, 2) => {
            SolutionResult::U32(aoc2024::day01::part_two(&aoc2024::day01::generator(input)))
        }
        (2024, 2, 1) => {
            SolutionResult::U32(aoc2024::day02::part_one(&aoc2024::day02::generator(input)))
        }
        (2024, 2, 2) => {
            SolutionResult::U32(aoc2024::day02::part_two(&aoc2024::day02::generator(input)))
        }
        (2024, 3, 1) => {
            SolutionResult::U32(aoc2024::day03::part_one(&aoc2024::day03::generator(input)))
        }
        (2024, 3, 2) => {
            SolutionResult::U32(aoc2024::day03::part_two(&aoc2024::day03::generator(input)))
        }
        (2024, 4, 1) => {
            SolutionResult::U32(aoc2024::day04::part_one(&aoc2024::day04::generator(input)))
        }
        (2024, 4, 2) => {
            SolutionResult::U32(aoc2024::day04::part_two(&aoc2024::day04::generator(input)))
        }
        (2024, 5, 1) => {
            SolutionResult::U32(aoc2024::day05::part_one(&aoc2024::day05::generator(input)))
        }
        (2024, 5, 2) => {
            SolutionResult::U32(aoc2024::day05::part_two(&aoc2024::day05::generator(input)))
        }
        (2024, 6, 1) => {
            SolutionResult::U32(aoc2024::day06::part_one(&aoc2024::day06::generator(input)))
        }
        (2024, 6, 2) => {
            SolutionResult::U32(aoc2024::day06::part_two(&aoc2024::day06::generator(input)))
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No solution available for {year} day {day} part {part}",
            ))
        }
    };
    let result_string = match result {
        SolutionResult::U32(value) => value.to_string(),
        SolutionResult::U64(value) => value.to_string(),
    };

    Ok(result_string)
}
