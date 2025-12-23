use std::ops::Range;

pub struct PuzzleInput {
    ranges: Vec<Range<u64>>,
}

pub fn generator(input: &str) -> PuzzleInput {
    let ranges = input
        .trim()
        .split(',')
        .map(|id_range| {
            let (start, end) = id_range.split_once('-').expect("invalid range format");
            let start: u64 = start.parse().expect("invalid number");
            let end: u64 = end.parse().expect("invalid number");
            start..end + 1
        })
        .collect();

    PuzzleInput { ranges }
}

pub fn part_one(input: PuzzleInput) -> u64 {
    input
        .ranges
        .into_iter()
        .flat_map(|r| r)
        .filter(|x| {
            let string = x.to_string();
            string.len() % 2 == 0 && string[0..string.len() / 2] == string[string.len() / 2..]
        })
        .sum()
}

pub fn part_two(input: PuzzleInput) -> u64 {
    input
        .ranges
        .into_iter()
        .flat_map(|r| r)
        .filter(|&x| {
            let s = x.to_string();
            let n = s.len();

            for k in 1..=n / 2 {
                if n % k != 0 {
                    continue;
                }
                let pattern = &s[0..k];
                if pattern.repeat(n / k) == s {
                    return true;
                }
            }

            false
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_one(generator(test_input)), 1227775554);
    }
}
