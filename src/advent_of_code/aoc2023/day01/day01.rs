pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        sum += format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = &line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let numbers: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        sum += format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        assert_eq!(part_one("1abc2"), 12)
    }

    #[test]
    fn test_part_one_example_two() {
        assert_eq!(part_one("pqr3stu8vwx"), 38)
    }

    #[test]
    fn test_part_one_example_three() {
        assert_eq!(part_one("a1b2c3d4e5f"), 15)
    }
    #[test]
    fn test_part_one_example_four() {
        assert_eq!(part_one("treb7uchet"), 77)
    }
}
