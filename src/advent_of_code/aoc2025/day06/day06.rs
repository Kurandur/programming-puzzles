use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Worksheet {
    length: usize,
    problems: Vec<Problem>,
}

impl Worksheet {
    pub fn result(&self) -> u64 {
        self.problems.iter().fold(0, |acc, p| acc + p.result())
    }

    pub fn from_str(input: &str) -> Self {
        let length = input
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .count();

        let mut problems: Vec<Problem> = input
            .lines()
            .last()
            .unwrap_or("")
            .split_whitespace()
            .filter_map(|o| {
                let operation = o.parse::<Operation>().ok()?;
                Some(Problem {
                    numbers: Vec::new(),
                    operation,
                })
            })
            .collect();

        for line in input.lines().take(input.lines().count().saturating_sub(1)) {
            for (i, number) in line
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .enumerate()
            {
                if let Some(problem) = problems.get_mut(i) {
                    problem.numbers.push(number);
                }
            }
        }

        Worksheet { length, problems }
    }

    pub fn from_str_transposed(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Worksheet {
                length: 0,
                problems: Vec::new(),
            };
        }

        let num_rows = lines.len();
        let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        let mut grid: Vec<Vec<char>> = vec![vec![' '; num_cols]; num_rows];
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                grid[r][c] = ch;
            }
        }

        let mut problem_ranges = Vec::new();
        let mut start: Option<usize> = None;

        for c in 0..num_cols {
            if grid.iter().all(|row| row[c] == ' ') {
                if let Some(s) = start {
                    problem_ranges.push(s..c);
                    start = None;
                }
            } else if start.is_none() {
                start = Some(c);
            }
        }
        if let Some(s) = start {
            problem_ranges.push(s..num_cols);
        }

        let mut problems = Vec::new();
        for range in problem_ranges.iter().rev() {
            let operator_row = num_rows - 1;
            let num_columns_in_problem = range.end - range.start;
            let mut numbers = vec![0u32; num_columns_in_problem];

            for r in 0..operator_row {
                for (col_idx, c) in (range.start..range.end).enumerate() {
                    let ch = grid[r][c];
                    if ch.is_digit(10) {
                        numbers[col_idx] = numbers[col_idx] * 10 + ch.to_digit(10).unwrap();
                    }
                }
            }

            let operator_char = grid[operator_row][range.start];
            let operation = match operator_char {
                '+' => Operation::Add,
                '*' => Operation::Multiply,
                _ => panic!("Unknown operator '{}'", operator_char),
            };

            problems.push(Problem { numbers, operation });
        }

        Worksheet {
            length: problems.len(),
            problems,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    numbers: Vec<u32>,
    operation: Operation,
}

impl Problem {
    pub fn result(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().map(|&n| n as u64).sum(),
            Operation::Multiply => self.numbers.iter().map(|&n| n as u64).product(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: String) -> u64 {
    Worksheet::from_str(&input).result()
}

pub fn part_two(input: String) -> u64 {
    Worksheet::from_str_transposed(&input).result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_one(generator(test_input)), 4277556);
    }

    #[test]
    fn test_part_two() {
        let test_input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_two(generator(test_input)), 3263827);
    }
}
