use aoc_runner_derive::{aoc, aoc_generator};

type InputType = (u8, u8);
type SolutionType = i32;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<InputType> { 
    input.lines()
        .map(|line| (line.as_bytes()[0] - b'A', line.as_bytes()[2] - b'X'))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(other, mine)| -> SolutionType { mine as SolutionType + 1 + score(other, mine) })
        .sum()
}

fn score(other: u8, mine: u8) -> i32 {
    match (other as i8 - mine as i8).rem_euclid(3) {
        0 => 3,
        2 => 6,
        _ => 0
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(other, outcome)| -> SolutionType { 
            ((other + outcome + 2).rem_euclid(3)) as SolutionType + 1 + score(other, (other + outcome + 2).rem_euclid(3)) 
        })
        .sum()
}