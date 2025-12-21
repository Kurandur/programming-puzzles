use std::u32;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let direction = line.as_bytes()[0];
        let number = line[1..].parse::<i32>().unwrap();
        match direction {
            b'L' => {
                position = ((position + 100) - number) % 100;
            }
            b'R' => position = (position + number) % 100,
            _ => {}
        }
        if position == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

pub fn part_two(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let direction = line.as_bytes()[0];
        let number = line[1..].parse::<i32>().unwrap();
        match direction {
            b'R' => {
                zero_count += (position + number) / 100;
                position = (position + number) % 100
            }
            b'L' => {
                let reversed = (100 - position) % 100;
                zero_count += (reversed + number) / 100;
                position = 100 + position - number % 100;
            }
            _ => {}
        }
    }
    zero_count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part_one(test_input), 3)
    }
}
