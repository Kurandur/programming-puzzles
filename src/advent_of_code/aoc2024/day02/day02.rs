use std::u32;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn check_safety(numbers: &Vec<u32>) -> bool {
    if numbers
        .windows(2)
        .all(|w| w[0] < w[1] && w[1].abs_diff(w[0]) <= 3)
    {
        return true;
    } else if numbers
        .windows(2)
        .all(|w| w[0] > w[1] && w[1].abs_diff(w[0]) <= 3)
    {
        return true;
    }
    return false;
}

pub fn part_one(input: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        if check_safety(&numbers) {
            safe_reports = safe_reports + 1
        }
    }
    safe_reports
}

pub fn part_two(input: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        if check_safety(&numbers) {
            safe_reports = safe_reports + 1
        } else {
            for i in 0..numbers.len() {
                let mut modified = numbers.clone();
                modified.remove(i);
                if check_safety(&modified) {
                    safe_reports = safe_reports + 1;
                    break;
                }
            }
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
