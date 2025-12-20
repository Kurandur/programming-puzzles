use std::u32;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(l), Ok(r)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                left.push(l);
                right.push(r);
            }
        }
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .map(|(l, r)| l - r)
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(l), Ok(r)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                left.push(l);
                right.push(r);
            }
        }
    }

    for &number in &left {
        let score = number * (right.iter().filter(|&&r| r == number).count() as u32);
        sum = sum + score;
    }

    sum
}

#[cfg(test)]
mod tests {}
