use aoc_runner_derive::{aoc, aoc_generator};

type InputType = ((i32, i32), (i32, i32));
type SolutionType = i32;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| -> InputType {
            let (lhs, rhs) = line.split_once(",").unwrap();
            let (llhs, lrhs) = rhs.split_once("-").unwrap();
            let (rlhs, rrhs) = lhs.split_once("-").unwrap();
            (
                (llhs.parse::<i32>().unwrap(), lrhs.parse::<i32>().unwrap()),
                (rlhs.parse::<i32>().unwrap(), rrhs.parse::<i32>().unwrap())    
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut count = 0;
    for couple in data.iter() {
        let l = couple.0;
        let r = couple.1;
        if (l.0 <= r.0 && l.1 >= r.1) || (r.0 <= l.0 && r.1 >= l.1) {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut count = 0;
    for couple in data.iter() {
        let l = couple.0;
        let r = couple.1;
        if (l.0 <= r.0 && (l.1 >= r.0 || l.1 >= r.1 )) 
            || (r.0 <= l.0 && (r.1 >= l.0 || r.1 >= l.1 ))
        {
            count += 1;
        }
    }
    count
}

