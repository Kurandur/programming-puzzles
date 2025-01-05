use std::{
    ops::{Add, BitXorAssign, Div},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Computer {
    register_a: u32,
    register_b: u32,
    register_c: u32,
    program: Vec<(Opcode, ComboOperator)>,
    output: Vec<u32>,
}

impl Computer {
    fn run_program(&mut self) -> Vec<u32> {
        let mut i = 0;

        loop {
            if let Some((opcode, combo_operator)) = &self.program.get(i) {
                opcode.execute(
                    &mut i,
                    &combo_operator,
                    &mut self.register_a,
                    &mut self.register_b,
                    &mut self.register_c,
                    &mut self.output,
                );
            } else {
                break;
            }
        }
        self.output.clone()
    }
    fn get_program_string(&self) -> String {
        self.program
            .iter()
            .map(|(opcode, combo_operator)| format!("{},{}", opcode, combo_operator))
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ComboOperator {
    LiteralValue(u8),
    RegisterAValue,
    RegisterBValue,
    RegisterCValue,
    Reserved,
}

impl ComboOperator {
    fn get_value(self, register_a: &u32, register_b: &u32, register_c: &u32) -> u32 {
        match self {
            ComboOperator::LiteralValue(value) => value.into(),
            ComboOperator::RegisterAValue => *register_a,
            ComboOperator::RegisterBValue => *register_b,
            ComboOperator::RegisterCValue => *register_c,
            ComboOperator::Reserved => unreachable!(),
        }
    }
}

impl std::fmt::Display for ComboOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ComboOperator::LiteralValue(v) => v,
                ComboOperator::RegisterAValue => 4,
                ComboOperator::RegisterBValue => 5,
                ComboOperator::RegisterCValue => 6,
                ComboOperator::Reserved => 7,
            }
        )
    }
}

impl FromStr for ComboOperator {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "0" | "1" | "2" | "3" => Ok(Self::LiteralValue(value.parse::<u8>().unwrap())),
            "4" => Ok(Self::RegisterAValue),
            "5" => Ok(Self::RegisterBValue),
            "6" => Ok(Self::RegisterCValue),
            "7" => Ok(Self::Reserved),
            _ => Err(format!("Unknown opcode: {}", value)),
        }
    }
}

impl Add<ComboOperator> for u32 {
    type Output = u32;

    fn add(self, rhs: ComboOperator) -> Self::Output {
        match rhs {
            ComboOperator::LiteralValue(value) => value as u32,
            ComboOperator::RegisterAValue => 4,
            ComboOperator::RegisterBValue => 5,
            ComboOperator::RegisterCValue => 6,
            _ => panic!("Addition is only supported for LiteralValue variants."),
        }
    }
}

impl Div<u32> for ComboOperator {
    type Output = u32;

    fn div(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            panic!("Division by zero is not allowed");
        }
        0 + self / rhs
    }
}

impl BitXorAssign<ComboOperator> for u32 {
    fn bitxor_assign(&mut self, rhs: ComboOperator) {
        *self ^= *self + rhs;
    }
}

#[derive(Debug, Clone)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Opcode {
    fn execute(
        &self,
        instruction_pointer: &mut usize,
        combo_operator: &ComboOperator,
        register_a: &mut u32,
        register_b: &mut u32,
        register_c: &mut u32,
        output: &mut Vec<u32>,
    ) {
        let combo_value = combo_operator.get_value(register_a, register_b, register_c);
        match self {
            Opcode::ADV => *register_a /= 2u32.pow(combo_value),
            Opcode::BXL => *register_b ^= *combo_operator,
            Opcode::BST => *register_b = combo_value % 8,
            Opcode::JNZ => {
                if *register_a != 0 {
                    *instruction_pointer = usize::try_from(0 + *combo_operator).unwrap() / 2;
                    return;
                }
            }
            Opcode::BXC => *register_b ^= *register_c,
            Opcode::OUT => output.push(combo_value % 8),
            Opcode::BDV => *register_b = *register_a / 2u32.pow(combo_value),
            Opcode::CDV => *register_c = *register_a / 2u32.pow(combo_value),
        }
        *instruction_pointer += 1;
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Opcode::ADV => 0,
                Opcode::BXL => 1,
                Opcode::BST => 2,
                Opcode::JNZ => 3,
                Opcode::BXC => 4,
                Opcode::OUT => 5,
                Opcode::BDV => 6,
                Opcode::CDV => 7,
            }
        )
    }
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "0" => Ok(Self::ADV),
            "1" => Ok(Self::BXL),
            "2" => Ok(Self::BST),
            "3" => Ok(Self::JNZ),
            "4" => Ok(Self::BXC),
            "5" => Ok(Self::OUT),
            "6" => Ok(Self::BDV),
            "7" => Ok(Self::CDV),
            _ => Err(format!("Unknown opcode: {}", value)),
        }
    }
}

pub fn generator(input: &str) -> Computer {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program: Vec<(Opcode, ComboOperator)> = Vec::new();

    for line in input.lines() {
        if line.starts_with("Register A:") {
            register_a = line["Register A:".len()..].trim().parse::<u32>().unwrap();
        } else if line.starts_with("Register B:") {
            register_b = line["Register B:".len()..].trim().parse::<u32>().unwrap();
        } else if line.starts_with("Register C:") {
            register_c = line["Register C:".len()..].trim().parse::<u32>().unwrap();
        } else if line.starts_with("Program:") {
            program = line["Program:".len()..]
                .trim()
                .split(',')
                .collect::<Vec<&str>>()
                .chunks(2)
                .map(|chunk| {
                    if let [opcode_str, combo_operator_str] = chunk {
                        let opcode = opcode_str
                            .parse::<Opcode>()
                            .unwrap_or_else(|e| panic!("Failed to parse Opcode: {}", e));
                        let combo_operator = combo_operator_str
                            .parse::<ComboOperator>()
                            .unwrap_or_else(|e| panic!("Failed to parse ComboOperator: {}", e));
                        (opcode, combo_operator)
                    } else {
                        panic!("Invalid program data: expected pairs, got {:?}", chunk);
                    }
                })
                .collect();
        }
    }

    Computer {
        register_a,
        register_b,
        register_c,
        program,
        output: Vec::new(),
    }
}

pub fn part_one(mut computer: Computer) -> String {
    computer
        .run_program()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part_two(computer: Computer) -> u32 {
    // Lets fucking bruteforce this!!!!!!!
    let target_output = computer.get_program_string();
    println!("{}", target_output);
    (0..u32::MAX)
        .find(|&i| {
            let mut computer_copy = computer.clone();
            computer_copy.register_a = i;
            computer_copy
                .run_program()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
                == target_output
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
