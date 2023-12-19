use core::fmt;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use solver::{MultiSolver, Solver};

pub mod solver;

mod cube_conundrum;
mod gear_ratios;
mod if_you_give_a_seed_a_fertilizer;
mod scratchcards;
mod trebuchet;
mod wait_for_it;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
enum Part {
    /// Puzzle part 1
    Part1,
    /// Puzzle part 2
    Part2,
    /// Full puzzle
    All,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
enum DayTitles {
    All,
    Trebuchet,
    CubeConundrum,
    GearRatios,
    Scratchcards,
    IfYouGiveASeedAFertilizer,
    WaitForIt,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Day {
    Numeric(u8),
    Name(DayTitles),
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Numeric(n) => write!(f, "Day {}", n),
            Day::Name(d) => write!(f, "{:?}", d),
        }
    }
}

fn valid_day(s: &str) -> Result<Day, clap::Error> {
    match s.parse::<u8>() {
        Ok(n) => {
            if n == 0 || n > 25 {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidValue,
                    format!("\n{}", day_values_error()),
                ));
            }
            Ok(Day::Numeric(n))
        }
        Err(_) => {
            let possible = DayTitles::from_str(s, true);
            match possible {
                Ok(d) => Ok(Day::Name(d)),
                Err(_) => Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidValue,
                    format!("\n{}", day_values_error()),
                )),
            }
        }
    }
}

fn possible_day_values() -> Vec<String> {
    vec!["1..24".to_string()]
        .into_iter()
        .chain(
            DayTitles::value_variants()
                .iter()
                .map(|d| format!("{}", d.to_possible_value().unwrap().get_name()))
                .collect::<Vec<String>>(),
        )
        .collect::<Vec<String>>()
}

fn possible_day_values_string(join_str: &str) -> String {
    possible_day_values().join(join_str)
}

fn day_values_error() -> String {
    format!("[possible values: {}]", possible_day_values_string(", "))
}

fn day_values_help() -> String {
    format!("Possible values:\n- {}", possible_day_values_string("\n- "))
}

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    #[arg(value_parser = valid_day, help = day_values_help())]
    day: Day,
    #[arg(value_enum)]
    part: Part,
    input: PathBuf,
}

fn run_day<P1: Solver, P2: Solver>(
    day_solver: Box<dyn MultiSolver<PartOne = P1, PartTwo = P2>>,
    part: Part,
    input: &PathBuf,
) -> Result<()> {
    match part {
        Part::Part1 => day_solver
            .get_part_one()
            .run(input, Some(day_solver.get_puzzle_title()))?,
        Part::Part2 => day_solver
            .get_part_two()
            .run(input, Some(day_solver.get_puzzle_title()))?,
        Part::All => day_solver.run_all(input)?,
    }
    Ok(())
}

fn find_runner(day: u8, part: Part, filepath: &PathBuf) -> Result<()> {
    match day {
        1 => run_day(Box::new(trebuchet::Trebuchet), part, filepath),
        2 => run_day(Box::new(cube_conundrum::CubeConundrum), part, filepath),
        3 => run_day(Box::new(gear_ratios::GearRatios), part, filepath),
        4 => run_day(Box::new(scratchcards::Scratchcards), part, filepath),
        5 => run_day(
            Box::new(if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer),
            part,
            filepath,
        ),
        6 => run_day(Box::new(wait_for_it::WaitForIt), part, filepath),
        _ => Err(anyhow!("Day {} not implemented", day)),
    }
}

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    println!(
        "User requested solution for {} (part: {:?})",
        cli.day, cli.part
    );

    match cli.day {
        Day::Numeric(n) => find_runner(n, cli.part, &cli.input)?,
        Day::Name(DayTitles::All) => {
            // run all days, input path is expected to be the base path
            // containing numbered directories (eg. 01, 02, 03, etc.)
            // with each containing the input file for that day called input with no extension
            for day in 1..=24 {
                let mut path = PathBuf::from(&cli.input);
                path.push(format!("{:02}", day));
                path.push("input");
                find_runner(day, cli.part, &path)?;
            }
        }
        Day::Name(DayTitles::Trebuchet) => find_runner(1, cli.part, &cli.input)?,
        Day::Name(DayTitles::CubeConundrum) => find_runner(2, cli.part, &cli.input)?,
        Day::Name(DayTitles::GearRatios) => find_runner(3, cli.part, &cli.input)?,
        Day::Name(DayTitles::Scratchcards) => find_runner(4, cli.part, &cli.input)?,
        Day::Name(DayTitles::IfYouGiveASeedAFertilizer) => find_runner(5, cli.part, &cli.input)?,
        Day::Name(DayTitles::WaitForIt) => find_runner(6, cli.part, &cli.input)?,
    };
    Ok(())
}
