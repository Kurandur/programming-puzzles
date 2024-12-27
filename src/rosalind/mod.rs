use utils::read_input;

use crate::utils::cli::RosalindProblem;

pub mod problems;
pub mod utils;

pub fn run_problem(problem: &RosalindProblem) -> Result<String, std::io::Error> {
    let input = &read_input(&problem);

    let result = match problem {
        RosalindProblem::Dna => problems::dna::solve(input),
        RosalindProblem::Rna => "No solution implemented!".to_string(),
        RosalindProblem::ReverseComplement => "No solution implemented!".to_string(),
    };
    Ok(result)
}
