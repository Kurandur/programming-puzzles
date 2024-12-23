use regex::Regex;

struct Button {
    x: i64,
    y: i64,
}

struct Prize {
    x: i64,
    y: i64,
}

pub struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

fn parse_input(input: String, delimiter: i64) -> Vec<ClawMachine> {
    let button_pattern = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut claw_machines = Vec::new();

    for block in input.split("\n\n") {
        let mut a = None;
        let mut b = None;
        let mut prize = None;

        for line in block.lines() {
            if let Some(captures) = button_pattern.captures(line) {
                let button_type = &captures[1];
                let x: i64 = captures[2].parse().unwrap();
                let y: i64 = captures[3].parse().unwrap();

                let button = Button { x, y };

                if button_type == "A" {
                    a = Some(button);
                } else if button_type == "B" {
                    b = Some(button);
                }
            } else if let Some(captures) = prize_pattern.captures(line) {
                let x: i64 = captures[1].parse().unwrap();
                let y: i64 = captures[2].parse().unwrap();

                prize = Some(Prize {
                    x: x + delimiter,
                    y: y + delimiter,
                });
            }
        }

        if let (Some(a), Some(b), Some(prize)) = (a, b, prize) {
            claw_machines.push(ClawMachine { a, b, prize });
        }
    }
    claw_machines
}

fn solve_claw_machine(claw_machine: ClawMachine, part_two: bool) -> Option<u64> {
    let determinant = claw_machine.a.x * claw_machine.b.y - claw_machine.a.y * claw_machine.b.x;

    if determinant == 0 {
        return None;
    }

    let a = (claw_machine.prize.x * claw_machine.b.y - claw_machine.prize.y * claw_machine.b.x)
        / determinant;
    let b = (claw_machine.a.x * claw_machine.prize.y - claw_machine.a.y * claw_machine.prize.x)
        / determinant;

    if !part_two && (a >= 100 || b >= 100) {
        return None;
    }

    if claw_machine.a.x * a + claw_machine.b.x * b != claw_machine.prize.x
        || claw_machine.a.y * a + claw_machine.b.y * b != claw_machine.prize.y
    {
        return None;
    }

    return Some((a * 3 + b) as u64);
}

pub fn part_one(input: String) -> u32 {
    let mut tokens = 0;
    let claw_machines = parse_input(input, 0);
    for claw_machine in claw_machines {
        if let Some(solution) = solve_claw_machine(claw_machine, false) {
            tokens += solution;
        }
    }
    tokens as u32
}

pub fn part_two(input: String) -> u64 {
    let mut tokens: u64 = 0;
    let claw_machines = parse_input(input, 10000000000000);
    for claw_machine in claw_machines {
        if let Some(solution) = solve_claw_machine(claw_machine, true) {
            tokens += solution as u64;
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
