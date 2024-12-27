use clap::{Parser, Subcommand, ValueEnum};

#[derive(Subcommand, Debug)]
pub enum Scaffold {
    Year {
        #[arg(value_parser = clap::value_parser!(u16).range(2014..2030))]
        year: u16,
    },
    Day {
        #[arg(value_parser = clap::value_parser!(u16).range(2014..2030))]
        year: u16,
        #[arg(value_parser = clap::value_parser!(u8).range(0..26))]
        day: u8,
    },
}

#[derive(Subcommand, Debug)]
pub enum SessionCommands {
    Set {
        #[arg()]
        token: String,
    },
    Remove,
}

#[derive(Subcommand, Debug)]
pub enum AocCommands {
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },
    Scaffold {
        #[command(subcommand)]
        r#type: Scaffold,
    },
    Download {
        #[arg(value_parser = clap::value_parser!(u16).range(2014..2030))]
        year: u16,
        #[arg(value_parser = clap::value_parser!(u8).range(0..26))]
        day: u8,
    },
    Run {
        #[arg(value_parser = clap::value_parser!(u16).range(2014..2030))]
        year: u16,
        #[arg(value_parser = clap::value_parser!(u8).range(0..26))]
        day: u8,
        #[arg(value_parser = clap::value_parser!(u8).range(0..3))]
        part: u8,
    },
    Solve {
        #[arg(value_parser = clap::value_parser!(u16).range(2014..2030))]
        year: u16,
        #[arg(value_parser = clap::value_parser!(u16).range(0..26))]
        day: u8,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RosalindProblem {
    #[clap(help = "Counting DNA Nucleotides")]
    Dna,
    #[clap(help = "Transcribing DNA into RNA")]
    Rna,
    #[clap(help = "Complementing a Strand of DNA")]
    Revc,
    #[clap(help = "Rabbits and Recurrence Relations ")]
    Fib,
}

#[derive(Subcommand, Debug)]
pub enum RosalindCommands {
    Run {
        #[arg(value_enum, help = "The Rosalind problem to solve")]
        problem: Option<RosalindProblem>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProgrammingPuzzles {
    #[command(
        about = "Advent of Code puzzles",
        long_about = "Advent of Code (AOC) is a series of programming challenges released daily during the month of December. Use this command to manage and solve AOC puzzles."
    )]
    Aoc {
        #[command(subcommand)]
        command: AocCommands,
    },
    #[command(about = "LeetCode puzzles")]
    LeetCode,
    #[command(about = "Project Euler puzzles")]
    ProjectEuler,
    #[command(about = "Rosalind bioinformatics challenges")]
    Rosalind {
        #[command(subcommand)]
        command: RosalindCommands,
    },
    #[command(about = "Everybody Codes puzzles")]
    EverybodyCodes,
}

#[derive(Parser)]
#[command(name = "programming-puzzles")]
#[command(author = "Marcel L. <4482510+mluettecke@users.noreply.github.com>")]
#[command(version = "0.01")]
#[command(about = "A helper cli to save some time while solving programming puzzles.")]
pub struct Cli {
    #[command(subcommand)]
    pub puzzle: Option<ProgrammingPuzzles>,
}
