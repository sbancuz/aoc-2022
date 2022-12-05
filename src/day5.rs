use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Vec<i32>;
type SolutionType = String;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<InputType> { 
    // skip the first lines because they contain the starting pos
    // i was to lazy to parse from the input so i just 
    // manually created it :^)
    input.lines().skip(10)
        .map(|str| -> InputType {
            let mut s_it = str.split(" ")
                                                        .skip(1)
                                                        .step_by(2);
            vec![s_it.next().unwrap().parse::<i32>().unwrap(),
                 s_it.next().unwrap().parse::<i32>().unwrap() - 1,
                 s_it.next().unwrap().parse::<i32>().unwrap() - 1
                ]
        }
    ).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut board = vec![
                            vec!['D', 'B', 'J', 'V'], 
                            vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
                            vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
                            vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
                            vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
                            vec!['R', 'D', 'B', 'S', 'N', 'G'],
                            vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
                            vec!['F', 'W', 'L'], 
                            vec!['S', 'V', 'F', 'M', 'R'], 
                            ];

    for instruction in data {
        for _ in 0..*instruction.get(0).unwrap() {
            let el = board.get_mut(*instruction.get(1).unwrap() as usize).unwrap()
                .pop().unwrap();
            board.get_mut(*instruction.get(2).unwrap() as usize).unwrap().push(el); 
        }
    }
    board.iter().map(|v| -> char {
        *v.last().unwrap()
    }).collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut board = vec![
        vec!['D', 'B', 'J', 'V'], 
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['F', 'W', 'L'], 
        vec!['S', 'V', 'F', 'M', 'R'], 
        ];
    for instruction in data {
        let mut els = vec![];
        for _ in 0..*instruction.get(0).unwrap() {
            els.insert(0, board.get_mut(*instruction.get(1).unwrap() as usize).unwrap()
                .pop().unwrap());
        }
        board.get_mut(*instruction.get(2).unwrap() as usize).unwrap().append(&mut els); 

    }
    board.iter().map(|v| -> char {
        *v.last().unwrap()
    }).collect()
}