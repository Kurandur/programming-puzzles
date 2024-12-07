use regex::Regex;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    for regex_match in regex.find_iter(input) {
        let (a, b): (u32, u32) = regex_match
            .as_str()
            .replace("mul(", "")
            .replace(")", "")
            .split_once(",")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();
        sum = sum + (a * b);
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let mut sum: u32 = 0;
    // more regex
    let regex = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut do_flag = true;
    for regex_match in regex.find_iter(input) {
        //println!("match: {}", regex_match.as_str());
        match regex_match.as_str() {
            "do()" => do_flag = true,
            "don't()" => do_flag = false,
            _ => {
                if do_flag {
                    let (a, b): (u32, u32) = regex_match
                        .as_str()
                        .replace("mul(", "")
                        .replace(")", "")
                        .split_once(",")
                        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                        .unwrap();
                    sum = sum + (a * b);
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
