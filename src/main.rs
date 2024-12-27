use std::process;

use clap::{CommandFactory, Parser};

use programming_puzzles::{
    rosalind::run_problem,
    utils::cli::{
        aoc::{download_input, run_day, scaffold_day, scaffold_year, solve_day, SessionManager},
        AocCommands, Cli, ProgrammingPuzzles, RosalindCommands, Scaffold, SessionCommands,
    },
};

fn main() {
    let cli = Cli::parse();
    let mut session_manager = SessionManager::new();

    match &cli.puzzle {
        Some(ProgrammingPuzzles::Aoc { command }) => match command {
            AocCommands::Scaffold { r#type } => match r#type {
                Scaffold::Day { year, day } => match scaffold_day(*year, *day) {
                    Ok(()) => {
                        println!("Successfully created day file")
                    }
                    Err(_e) => {
                        process::exit(1);
                    }
                },
                Scaffold::Year { year } => match scaffold_year(*year) {
                    Ok(()) => {
                        println!("Finished scaffolding year {}", year);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                },
            },
            AocCommands::Download { year, day } => match download_input(*year, *day) {
                Ok(()) => {
                    println!("Downloaded input for {} day {}", year, day)
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            AocCommands::Session { command } => match command {
                SessionCommands::Remove => {}
                SessionCommands::Set { token } => {
                    match session_manager.set_session_token(token.into()) {
                        Ok(_) => {
                            println!("Successfully set the aoc session token.");
                        }
                        Err(err) => {
                            eprintln!("Well well well!: {}", err)
                        }
                    }
                }
            },
            AocCommands::Run { year, day, part } => match run_day(*year, *day, *part) {
                Ok(result) => {
                    println!("The result for {} day {} part {} is:", year, day, part);
                    println!("{}", result)
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            AocCommands::Solve { year, day } => solve_day(*year, *day),
        },
        Some(ProgrammingPuzzles::LeetCode {}) => {
            println!("Leetcode was picked!")
        }
        Some(ProgrammingPuzzles::ProjectEuler {}) => {
            println!("Project euler was picked!")
        }
        Some(ProgrammingPuzzles::Rosalind { command }) => match command {
            RosalindCommands::Run { problem } => match problem {
                Some(selected_problem) => match run_problem(selected_problem) {
                    Ok(result) => {
                        println!("The result for problem {:?} is:", selected_problem);
                        println!("{}", result)
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                },
                None => {
                    println!("Please pick a problem to run. Here are the available problems:");
                }
            },
        },
        Some(ProgrammingPuzzles::EverybodyCodes {}) => {
            println!("Everybody codes was picked!")
        }
        None => {
            let mut cmd = Cli::command();
            let help_message = cmd.render_help().to_string();
            println!("Available options:");
            println!("{}", help_message);
        }
    }
}
