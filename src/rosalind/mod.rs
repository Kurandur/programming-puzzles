use utils::read_input;

use crate::utils::cli::RosalindProblem;

pub mod problems;
pub mod utils;

pub fn run_problem(problem: &RosalindProblem) -> Result<String, std::io::Error> {
    let input = &read_input(&problem);

    let result = match problem {
        RosalindProblem::Dna => problems::dna::solve(input),
        RosalindProblem::Rna => problems::rna::solve(input),
        RosalindProblem::Revc => problems::revc::solve(input),
        RosalindProblem::Fib => problems::fib::solve(input),
        RosalindProblem::Gc => problems::gc::solve(input),
        RosalindProblem::Hamm => problems::hamm::solve(input),
    };
    Ok(result)
}
