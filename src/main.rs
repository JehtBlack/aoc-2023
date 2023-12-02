use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenv::dotenv;

mod day01;
mod day02;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
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
    Trebuchet,
    CubeConundrum,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Day {
    Numeric(u8),
    Name(DayTitles),
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
                .map(|d| format!("{:?}", d).to_lowercase())
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

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    match cli.day {
        Day::Numeric(1) | Day::Name(DayTitles::Trebuchet) => match cli.part {
            Part::Part1 => day01::part1(&cli.input)?,
            Part::Part2 => day01::part2(&cli.input)?,
            Part::All => {
                day01::part1(&cli.input)?;
                day01::part2(&cli.input)?;
            }
        },
        Day::Numeric(2) | Day::Name(DayTitles::CubeConundrum) => match cli.part {
            Part::Part1 => day02::part1(&cli.input)?,
            Part::Part2 => day02::part2(&cli.input)?,
            Part::All => {
                day02::part1(&cli.input)?;
                day02::part2(&cli.input)?;
            }
        },
        _ => println!("Day {:?} not implemented", cli.day),
    }

    Ok(())
}
