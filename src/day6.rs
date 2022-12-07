<<<<<<< HEAD
use std::{vec, usize};

=======
>>>>>>> 0f35515 (Day6)
use aoc_runner_derive::{aoc, aoc_generator};

type InputType = String;
type SolutionType = i32;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> InputType { 
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut buff: [char; 4] = ['-', '-', '-', '-'];
    let mut pnt = 0;
    let mut res = 0;
    for c in data.chars() {
        
        buff[pnt] = c;
        pnt = (pnt + 1) % 4;    
        res += 1;
        if !has_dup(&buff) && res > 3 {
            break;
        }
    }
    res
}

fn has_dup(arr: &[char]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i].eq(&arr[j]) {            
                return true;
            }
        }   
    }
    false
}
#[aoc(day6, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut buff: [char; 14] = ['-'; 14];
    let mut pnt = 0;
    let mut res = 0;
    for c in data.chars() {
        
        buff[pnt] = c;
        pnt = (pnt + 1) % 14;    
        res += 1;
        if !has_dup(&buff) && res > 13 {
            break;
        }
    }
    res
}