use aoc_runner_derive::{aoc, aoc_generator};

type SolutionType = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<SolutionType> {
    let mut result = vec![];
    let mut sum = 0;
    for line in input.lines(){
        if line == "" {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[SolutionType]) -> SolutionType {
    *data.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[SolutionType]) -> SolutionType {
    let mut result = 0;
    let mut last_max = std::i32::MAX;
    let mut curr_max = 0;

    for _ in 0..3 {
        for v in data.iter() {
            if *v > curr_max && *v < last_max{
                curr_max = *v;
            }
        }

        last_max = curr_max;
        curr_max = 0;
        result += last_max;

    }


    result
}