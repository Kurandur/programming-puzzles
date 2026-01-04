use std::collections::{HashSet, VecDeque};

pub struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

impl Machine {
    pub fn min_button(&self) -> Option<u32> {
        let button_count = self.buttons.len();

        let mut best: Option<u32> = None;

        for mask in 0..(1u32 << button_count) {
            let mut acc = 0;
            let mut count = 0;

            for i in 0..button_count {
                if (mask >> i) & 1 == 1 {
                    acc ^= self.buttons[i];
                    count += 1;
                }
            }

            if acc == self.lights {
                best = match best {
                    Some(b) => Some(b.min(count)),
                    None => Some(count),
                };
            }
        }
        best
    }
}

pub fn generator(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    for line in input.lines() {
        let split_vec: Vec<_> = line.split_whitespace().collect();
        let lights_string = split_vec[0];
        let width = lights_string.len() as u32 - 2;

        let lights: u32 = lights_string[1..lights_string.len() - 1]
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Invalid light char"),
            })
            .fold(0, |acc, bit| (acc << 1) | bit);

        let buttons: Vec<u32> = split_vec[1..]
            .iter()
            .filter(|s| s.starts_with('('))
            .map(|token| {
                token
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .fold(0, |acc, idx| acc | (1 << (width - 1 - idx)))
            })
            .collect();
        let joltages: Vec<u32> = split_vec
            .iter()
            .find(|s| s.starts_with('{'))
            .map(|s| {
                s.trim_matches(|c| c == '{' || c == '}')
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect()
            })
            .unwrap_or_else(|| vec![]);

        machines.push(Machine {
            lights,
            buttons,
            joltages,
        })
    }
    machines
}

pub fn part_one(machines: Vec<Machine>) -> u32 {
    machines.iter().filter_map(|m| m.min_button()).sum()
}

pub fn part_two(machines: Vec<Machine>) -> u32 {
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
        assert_eq!(part_one(generator(test_input)), 7);
    }
}
