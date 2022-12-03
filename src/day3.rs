use aoc_runner_derive::{aoc, aoc_generator};

type InputType = String;
type SolutionType = i32;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| -> SolutionType {
            let lhs: &str;
            let rhs: &str;
            (lhs, rhs) = line.split_at(line.len() / 2);
            let dup = get_dup_char(lhs, rhs).unwrap();
            priority(dup)
        })
        .sum()
}

fn get_dup_char (str1: &str, str2: &str) -> Option<char> {
    for c1 in str1.chars(){
        for c2 in str2.chars(){
            if c1 == c2 {
                return Some(c1);
            }
        }
    }
    None
}

fn priority(c: char) -> SolutionType {
    if ('a'..='z').contains(&c) {
        1 + (c as u8 - b'a') as SolutionType
    } else {
        27 + (c as u8 - b'A') as SolutionType
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {

    let mut result = 0;
    for i in (0..data.len()).step_by(3) {
        for c in data[i].chars() {
            if data[i + 1].contains(c) && data[i + 2].contains(c) {
                result += priority(c);
                break;
            }
        }
    }
    result
}

