use anyhow::Result;
use std::path::PathBuf;

pub trait MultiSolver {
    type PartOne: Solver;
    type PartTwo: Solver;
    fn get_puzzle_title(&self) -> &str;
    fn get_part_one(&self) -> Self::PartOne;
    fn get_part_two(&self) -> Self::PartTwo;

    fn run_all(&self, filepath: &PathBuf) -> Result<()> {
        let part_one = self.get_part_one();
        let part_two = self.get_part_two();
        println!("{}", self.get_puzzle_title());
        part_one.run(filepath, None)?;
        part_two.run(filepath, None)?;
        Ok(())
    }
}

pub trait Solver {
    fn part_description(&self) -> (u32, &str);
    fn get_solution(&self, filepath: &PathBuf) -> Result<i32>;

    fn run(&self, filepath: &PathBuf, title: Option<&str>) -> Result<()> {
        if title.is_some() {
            println!("{}", title.unwrap());
        }
        let solution = self.get_solution(filepath)?;
        let (part, desc) = self.part_description();
        println!("[Part {}] {}: {}", part, desc, solution);
        Ok(())
    }
}
