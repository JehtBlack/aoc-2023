use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    path::PathBuf,
};

use anyhow::{anyhow, Result};

use crate::solver::{MultiSolver, Solver};

///     --- Day 3: Gear Ratios ---
///
/// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
///
///      --- Part Two ---
///
/// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
/// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub struct GearRatios;
pub struct PartOne;
pub struct PartTwo;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SchematicComponent {
    PartNumber(i32),
    Symbol(char),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PositionalSchematicComponent {
    pub component: SchematicComponent,
    pub line: usize,
    pub position: usize,
    pub length: usize,
}

impl MultiSolver for GearRatios {
    type PartOne = PartOne;
    type PartTwo = PartTwo;

    fn get_puzzle_title(&self) -> &str {
        "Day 3: Gear Ratios"
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
        (1, "Sum of part numbers")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        let mut schematic_components: BTreeSet<PositionalSchematicComponent> = BTreeSet::new();

        let mut prev_line: Option<Vec<PositionalSchematicComponent>> = None;
        for (line_num, line) in read_to_string(filepath)?.lines().enumerate() {
            let mut current_line: Vec<PositionalSchematicComponent> = vec![];

            let mut pos: usize = 0;
            for possible_component in extract_schematic_line_parts(line) {
                let len = possible_component.len();
                if !possible_component.contains('.') {
                    let comp = possible_component
                        .parse::<i32>()
                        .map_or(SchematicComponent::Symbol('*'), |n| {
                            SchematicComponent::PartNumber(n)
                        });
                    current_line.push(PositionalSchematicComponent {
                        component: comp,
                        line: line_num,
                        position: pos,
                        length: len,
                    });
                }
                pos += len;
            }

            for (line_pos, component) in current_line.iter().enumerate() {
                match component.component {
                    SchematicComponent::PartNumber(_) => {
                        // look left, look right, can this number validate itself ?
                        let mut valid = false;
                        if line_pos > 0 {
                            let prev_component = &current_line[line_pos - 1];
                            match prev_component.component {
                                SchematicComponent::Symbol(_) => {
                                    if prev_component.position == component.position - 1 {
                                        valid = true;
                                    }
                                }
                                _ => {}
                            }
                        }

                        if line_pos < current_line.len() - 1 {
                            let next_component = &current_line[line_pos + 1];
                            match next_component.component {
                                SchematicComponent::Symbol(_) => {
                                    if next_component.position
                                        == component.position + component.length
                                    {
                                        valid = true;
                                    }
                                }
                                _ => {}
                            }
                        }

                        // look up to the previous line, can this number validate itself ?
                        if let Some(prev_line) = &prev_line {
                            for prev_component in prev_line {
                                if prev_component.position
                                    >= component.position.checked_sub(1).unwrap_or(0)
                                    && prev_component.position
                                        <= component.position + component.length
                                {
                                    // within range
                                    match prev_component.component {
                                        SchematicComponent::Symbol(_) => {
                                            valid = true;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }

                        if valid {
                            schematic_components.insert(component.clone());
                        }
                    }
                    SchematicComponent::Symbol(_) => {
                        // look up to the previous line, can this symbol validate any numbers ?
                        if let Some(prev_line) = &mut prev_line {
                            for prev_component in prev_line.iter() {
                                if !schematic_components.contains(prev_component)
                                    && component.position
                                        >= prev_component.position.checked_sub(1).unwrap_or(0)
                                    && component.position
                                        <= prev_component.position + prev_component.length
                                {
                                    // within range
                                    match prev_component.component {
                                        SchematicComponent::PartNumber(_) => {
                                            schematic_components.insert(prev_component.clone());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            prev_line = Some(current_line);
        }

        let nums: Vec<i32> = schematic_components
            .iter()
            .fold(vec![], |mut acc, component| {
                match component.component {
                    SchematicComponent::PartNumber(n) => {
                        acc.push(n);
                    }
                    _ => {}
                }
                acc
            });
        Ok(nums.iter().sum::<i32>())
    }
}

impl Solver for PartTwo {
    fn part_description(&self) -> (u32, &str) {
        (2, "Sum of gear ratios")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        let mut gears: BTreeMap<(usize, usize), Vec<i32>> = BTreeMap::new();
        let mut prev_line: Option<Vec<PositionalSchematicComponent>> = None;
        for (line_num, line) in read_to_string(filepath)?.lines().enumerate() {
            let mut current_line: Vec<PositionalSchematicComponent> = vec![];

            let mut pos: usize = 0;
            for possible_component in extract_schematic_line_parts(line) {
                let len = possible_component.len();
                if !possible_component.contains('.') {
                    let comp = possible_component.parse::<i32>().map_or(
                        SchematicComponent::Symbol(
                            possible_component
                                .chars()
                                .nth(0)
                                .ok_or(anyhow!("Single char symbol parsed as 0 size string!"))?,
                        ),
                        |n| SchematicComponent::PartNumber(n),
                    );
                    current_line.push(PositionalSchematicComponent {
                        component: comp,
                        line: line_num,
                        position: pos,
                        length: len,
                    });
                }
                pos += len;
            }

            for (component_index, component) in current_line.iter().enumerate() {
                match component.component {
                    SchematicComponent::Symbol('*') => {
                        // look left, look right, are the ratios present ?
                        let left: Option<i32> = if component_index > 0 {
                            let prev_component = &current_line[component_index - 1];
                            match prev_component.component {
                                SchematicComponent::PartNumber(n) => {
                                    if prev_component.position + prev_component.length
                                        == component.position
                                    {
                                        Some(n)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        };

                        let right: Option<i32> = if component_index < current_line.len() - 1 {
                            let next_component = &current_line[component_index + 1];
                            match next_component.component {
                                SchematicComponent::PartNumber(n) => {
                                    if next_component.position <= component.position + 1 {
                                        Some(n)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        };

                        // look up to the previous line, are there ratios ?
                        let ratios: Vec<i32> = {
                            let mut ratios: Vec<i32> = vec![];
                            if left.is_some() {
                                ratios.push(left.unwrap());
                            }
                            if right.is_some() {
                                ratios.push(right.unwrap());
                            }
                            if let Some(prev_line) = &prev_line {
                                for prev_component in prev_line {
                                    if prev_component.position + prev_component.length
                                        >= component.position
                                        && prev_component.position <= component.position + 1
                                    {
                                        // within range
                                        match prev_component.component {
                                            SchematicComponent::PartNumber(n) => {
                                                ratios.push(n);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            ratios
                        };
                        gears.insert((line_num, component.position), ratios);
                    }
                    SchematicComponent::PartNumber(n) => {
                        // look above for a gear that this is a ratio for
                        if let Some(prev_line) = &prev_line {
                            for prev_component in prev_line {
                                if prev_component.position
                                    >= component.position.checked_sub(1).unwrap_or(0)
                                    && prev_component.position
                                        <= component.position + component.length
                                {
                                    // within range
                                    match prev_component.component {
                                        SchematicComponent::Symbol('*') => {
                                            gears
                                                .get_mut(&(
                                                    prev_component.line,
                                                    prev_component.position,
                                                ))
                                                .map(|ratios| {
                                                    ratios.push(n);
                                                });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            prev_line = Some(current_line);
        }

        let ratios = gears.iter().fold(vec![], |mut acc, (_, ratios)| {
            if ratios.len() == 2 {
                acc.push(ratios.iter().product::<i32>());
            }
            acc
        });

        Ok(ratios.iter().sum::<i32>())
    }
}

fn extract_schematic_line_parts(s: &str) -> Vec<&str> {
    let mut parts: Vec<&str> = vec![];

    #[derive(PartialEq, Eq)]
    enum CharType {
        Numeric(usize),
        Dot(usize),
        Symbol,
    }

    let mut last_char_type: Option<CharType> = None;
    for (i, c) in s.chars().enumerate() {
        let current_char_type = if c.is_numeric() {
            CharType::Numeric(i)
        } else if c == '.' {
            CharType::Dot(i)
        } else {
            CharType::Symbol
        };

        last_char_type
            .as_ref()
            .map(|last_char_type| match last_char_type {
                CharType::Numeric(start) => match current_char_type {
                    CharType::Numeric(_) => {}
                    _ => parts.push(&s[*start..i]),
                },
                CharType::Dot(start) => match current_char_type {
                    CharType::Dot(_) => {}
                    _ => parts.push(&s[*start..i]),
                },
                CharType::Symbol => {}
            });

        if current_char_type == CharType::Symbol {
            parts.push(&s[i..=i]);
        }

        match current_char_type {
            CharType::Numeric(_) => match last_char_type {
                Some(CharType::Numeric(_)) => {}
                _ => last_char_type = Some(current_char_type),
            },
            CharType::Dot(_) => match last_char_type {
                Some(CharType::Dot(_)) => {}
                _ => last_char_type = Some(current_char_type),
            },
            _ => last_char_type = Some(current_char_type),
        }
    }

    if let Some(last_char_type) = last_char_type {
        match last_char_type {
            CharType::Numeric(start) => parts.push(&s[start..]),
            CharType::Dot(start) => parts.push(&s[start..]),
            _ => {}
        }
    }
    parts
}
