use clap::{Parser, Subcommand};

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

#[derive(Subcommand, Debug)]
pub enum ProgrammingPuzzles {
    Aoc {
        #[command(subcommand)]
        command: AocCommands,
    },
    LeetCode {},
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
