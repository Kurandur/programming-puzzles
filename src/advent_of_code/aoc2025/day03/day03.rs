use std::u32;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut max = 0;
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).expect("Yo error") as u8)
            .collect();
        for (i, &d1) in digits.iter().enumerate() {
            for &d2 in &digits[i + 1..] {
                let candidate = d1 * 10 + d2;
                if candidate > max {
                    max = candidate
                }
            }
        }
        sum += max as u32
    }
    sum
}
pub fn max_k_subsequence(digits: &[u8], k: usize) -> Vec<u8> {
    let mut res = Vec::with_capacity(k);

    for (i, &d) in digits.iter().enumerate() {
        while let Some(&last) = res.last() {
            if last < d && res.len() + (digits.len() - i) > k {
                res.pop();
            } else {
                break;
            }
        }
        if res.len() < k {
            res.push(d);
        }
    }

    res
}

pub fn part_two(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut max_digits = Vec::with_capacity(12);

        for (i, &d) in digits.iter().enumerate() {
            while let Some(&last) = max_digits.last() {
                if last < d && max_digits.len() + (digits.len() - i) > 12 {
                    max_digits.pop();
                } else {
                    break;
                }
            }
            if max_digits.len() < 12 {
                max_digits.push(d);
            }
        }

        let mut max_val: u64 = 0;
        for &d in &max_digits {
            max_val = max_val * 10 + d as u64;
        }

        sum += max_val;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(part_one(&generator(test_input)), 357);
    }
}
