use std::collections::HashMap;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u64 {
    let numbers = input
        .split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sum = 0;
    let mut memo: HashMap<(u64, usize), u64> = HashMap::new();

    for n in numbers {
        sum += count(n, 3, &mut memo);
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let numbers = input
        .split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sum = 0;
    let mut memo: HashMap<(u64, usize), u64> = HashMap::new();

    for n in numbers {
        sum += count(n, 75, &mut memo);
    }

    sum
}

/*
 */
fn count(number: u64, steps: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(&cached) = memo.get(&(number, steps)) {
        return cached;
    }

    if steps == 0 {
        return 1;
    }

    let digit_count = ((number as f64).log10().floor() as usize) + 1;
    let res = match number {
        0 => count(1, steps - 1, memo),
        _ if digit_count % 2 == 0 => {
            let split_point = 10_u64.pow((digit_count / 2) as u32);
            let left = number / split_point;
            let right = number % split_point;
            count(left, steps - 1, memo) + count(right, steps - 1, memo)
        }
        _ => count(number * 2024, steps - 1, memo),
    };

    memo.insert((number, steps), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
