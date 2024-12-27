fn fib(n: u32, k: u32) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut dp = vec![0u64; (n + 1) as usize];
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        dp[i as usize] = dp[(i - 1) as usize] + k as u64 * dp[(i - 2) as usize];
    }

    dp[n as usize]
}

pub fn solve(input: &str) -> String {
    let mut parts = input.split_whitespace();

    let first = match parts.next() {
        Some(part) => match part.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return "Invalid first number".to_string(),
        },
        None => return "Missing first number".to_string(),
    };

    let second = match parts.next() {
        Some(part) => match part.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return "Invalid second number".to_string(),
        },
        None => return "Missing second number".to_string(),
    };

    format!("{}", fib(first, second))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = "5 3";
        assert_eq!(solve(sample_data), "19")
    }
}
