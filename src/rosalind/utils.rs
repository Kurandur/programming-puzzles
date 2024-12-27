use std::fs;

use crate::utils::cli::RosalindProblem;

pub fn read_input(problem: &RosalindProblem) -> String {
    let problem_name = format!("{:?}", problem).to_lowercase();
    let path = format!("src/rosalind/data/rosalind_{}.txt", problem_name);
    fs::read_to_string(path.clone()).expect(&format!("Unable to read input from: {}", path))
}
