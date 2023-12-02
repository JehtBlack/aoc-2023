use anyhow::{anyhow, Result};
use std::{fs::read_to_string, path::PathBuf};

use crate::solver::{MultiSolver, Solver};

///     --- Day 1: Trebuchet?! ---
///
/// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
///
/// For example:
/// ```
/// 1abc2  
/// pqr3stu8vwx  
/// a1b2c3d4e5f  
/// treb7uchet  
/// ```
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
///
///     --- Part Two ---
///
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
/// ```
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
///
/// What is the sum of all of the calibration values?
pub struct Trebuchet;
pub struct PartOne;
pub struct PartTwo;

impl MultiSolver for Trebuchet {
    type PartOne = PartOne;
    type PartTwo = PartTwo;

    fn get_puzzle_title(&self) -> &str {
        "Day 1: Trebuchet?!"
    }

    fn get_part_one(&self) -> Self::PartOne {
        PartOne
    }

    fn get_part_two(&self) -> Self::PartTwo {
        PartTwo
    }
}

impl Solver for PartOne {
    fn part_description(&self) -> (u32, &str) {
        (1, "Sum of calibration values")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        let mut result = 0;
        for line in read_to_string(filepath)?.lines() {
            let left = line.find(char::is_numeric).ok_or(anyhow!(
                "Couldn't find a digit in the input string '{}'",
                line
            ))?;
            let right = line.rfind(char::is_numeric).ok_or(anyhow!(
                "Couldn't find a digit in the input string '{}'",
                line
            ))?;
            let concatenated =
                (10 * line[left..=left].parse::<i32>()?) + line[right..=right].parse::<i32>()?;
            result += concatenated;
        }
        Ok(result)
    }
}

impl Solver for PartTwo {
    fn part_description(&self) -> (u32, &str) {
        (2, "Sum of calibration values")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        fn extract_digit(s: &str) -> Option<i32> {
            let len = s.len();
            for (i, c) in s.chars().enumerate() {
                match c {
                    'o' => {
                        if i + 3 <= len && &s[i..i + 3] == "one" {
                            return Some(1);
                        }
                    }
                    't' => {
                        if i + 3 <= len && &s[i..i + 3] == "two" {
                            return Some(2);
                        } else if i + 5 <= len && &s[i..i + 5] == "three" {
                            return Some(3);
                        }
                    }
                    'f' => {
                        if i + 4 <= len && &s[i..i + 4] == "four" {
                            return Some(4);
                        } else if i + 4 <= len && &s[i..i + 4] == "five" {
                            return Some(5);
                        }
                    }
                    's' => {
                        if i + 3 <= len && &s[i..i + 3] == "six" {
                            return Some(6);
                        } else if i + 5 <= len && &s[i..i + 5] == "seven" {
                            return Some(7);
                        }
                    }
                    'e' => {
                        if i + 5 <= len && &s[i..i + 5] == "eight" {
                            return Some(8);
                        }
                    }
                    'n' => {
                        if i + 4 <= len && &s[i..i + 4] == "nine" {
                            return Some(9);
                        }
                    }
                    _ => {
                        if c.is_numeric() {
                            return Some(c.to_digit(10).expect("This character can't be converted to a digit despite testing as numeric...") as i32);
                        }
                    }
                }
            }
            return None;
        }

        fn rextract_digit(s: &str) -> Option<i32> {
            let len = s.len();
            for (i, c) in s.chars().rev().enumerate() {
                match c {
                    'e' => {
                        if len - i >= 3 && &s[len - i - 3..len - i] == "one" {
                            return Some(1);
                        } else if len - i >= 5 && &s[len - i - 5..len - i] == "three" {
                            return Some(3);
                        } else if len - i >= 4 && &s[len - i - 4..len - i] == "five" {
                            return Some(5);
                        } else if len - i >= 4 && &s[len - i - 4..len - i] == "nine" {
                            return Some(9);
                        }
                    }
                    'o' => {
                        if len - i >= 3 && &s[len - i - 3..len - i] == "two" {
                            return Some(2);
                        }
                    }
                    'r' => {
                        if len - i >= 4 && &s[len - i - 4..len - i] == "four" {
                            return Some(4);
                        }
                    }
                    'x' => {
                        if len - i >= 3 && &s[len - i - 3..len - i] == "six" {
                            return Some(6);
                        }
                    }
                    'n' => {
                        if len - i >= 5 && &s[len - i - 5..len - i] == "seven" {
                            return Some(7);
                        }
                    }
                    't' => {
                        if len - i >= 5 && &s[len - i - 5..len - i] == "eight" {
                            return Some(8);
                        }
                    }
                    _ => {
                        if c.is_numeric() {
                            return Some(c.to_digit(10).expect("This character can't be converted to a digit despite testing as numeric...") as i32);
                        }
                    }
                }
            }
            return None;
        }

        let mut result = 0;
        for line in read_to_string(filepath)?.lines() {
            let left = extract_digit(line).ok_or(anyhow!(
                "Couldn't find a number (digit or spelled) in the input string '{}'",
                line
            ))?;
            let right = rextract_digit(line).ok_or(anyhow!(
                "Couldn't find a number (digit or spelled) in the input string '{}'",
                line
            ))?;
            result += (10 * left) + right;
        }
        Ok(result)
    }
}
