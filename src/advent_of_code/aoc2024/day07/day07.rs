pub struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

fn is_equation_solveable_part_one(equation: &Equation) -> bool {
    let n = equation.numbers.len();
    if n == 0 {
        return false;
    }
    if n == 1 {
        return equation.numbers[0] == equation.test_value;
    }

    let num_combinations = 1 << (n - 1);
    for combination in 0..num_combinations {
        let mut result = equation.numbers[0];
        for (i, &num) in equation.numbers[1..].iter().enumerate() {
            if (combination & (1 << i)) != 0 {
                result += num;
            } else {
                result *= num;
            }
        }
        if result == equation.test_value {
            return true;
        }
    }
    false
}

fn is_equation_solveable_part_two(equation: &Equation) -> bool {
    let n = equation.numbers.len();
    if n == 0 {
        return false;
    }
    if n == 1 {
        return equation.numbers[0] == equation.test_value;
    }

    let num_combinations = 3usize.pow((n - 1) as u32);
    for combination in 0..num_combinations {
        let mut result = equation.numbers[0];
        let mut temp_combination = combination;

        for (_i, &num) in equation.numbers[1..].iter().enumerate() {
            match temp_combination % 3 {
                0 => {
                    result += num;
                }
                1 => {
                    result *= num;
                }
                2 => {
                    result = format!("{}{}", result, num).parse::<u64>().unwrap();
                }
                _ => unreachable!(),
            }
            temp_combination /= 3;
        }

        if result == equation.test_value {
            return true;
        }
    }
    false
}

pub fn generator(input: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        if let (Some(test_value_part), Some(numbers_part)) = (parts.next(), parts.next()) {
            if let Ok(test_value) = test_value_part.trim().parse::<u64>() {
                let numbers: Vec<u64> = numbers_part
                    .trim()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<u64>().ok())
                    .collect();
                equations.push(Equation {
                    test_value,
                    numbers,
                });
            }
        }
    }
    equations
}

pub fn part_one(input: Vec<Equation>) -> u64 {
    let mut sum: u64 = 0;
    for equation in input {
        if is_equation_solveable_part_one(&equation) {
            sum += equation.test_value;
        }
    }

    sum
}

pub fn part_two(input: Vec<Equation>) -> u64 {
    let mut sum: u64 = 0;
    for equation in input {
        if is_equation_solveable_part_two(&equation) {
            sum += equation.test_value;
        }
    }

    sum
}

#[cfg(test)]
mod tests {}
