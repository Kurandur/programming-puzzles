pub struct Machine {}

pub fn generator(input: &str) -> String {
    for line in input.lines() {
        println!("Parsing: {}", line);
    }
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    u32::MAX
}

pub fn part_two(input: &str) -> u32 {
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let parsed_input = generator(test_input);
        println!("{}", parsed_input);
        assert_eq!(1, 1);
    }
}
