use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Node;

#[derive(Debug, Clone)]
pub enum Node {
    Dir(String, Vec<Node>, i32),
    File(String, i32),
}
use Node::*;

impl Node {
    pub fn insert(&mut self, n: Node) {
        match self {
            Dir(_, ref mut dirs, ref mut sz) => {
                *sz += n.clone().get_size();
                dirs.push(n);
            }
            _ => unreachable!(),
        }
    }

    fn get_size(&self) -> i32 {
        match self {
            Dir(_, _, sz) => *sz,
            File(_, sz) => *sz,
        }
    }
}

impl IntoIterator for Node {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        fn append(nod: Node, v: &mut Vec<Node>) {
            v.push(nod.clone());

            if let Dir(_, dirs, _) = nod {
                for d in dirs {
                    append(d, v);
                }
            }
        }

        let mut result = vec![];
        append(self, &mut result);
        result.into_iter()
    }
}
type SolutionType = i32;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> InputType {
    let mut iter = input.lines().skip(1);
    generate_tree(&mut iter, "/".to_string())
}

pub fn generate_tree(lines: &mut dyn Iterator<Item = &str>, name: String) -> Node {
    let mut n = Dir(name, vec![], 0);

    while let Some(line) = lines.next() {
        let mut itok = line.split(" ");
        let mut tok = itok.next().unwrap();
        match tok {
            "$" => {
                tok = itok.next().unwrap();
                match tok {
                    "cd" => {
                        tok = itok.next().unwrap();
                        if tok == ".." {
                            return n;
                        }
                        n.insert(generate_tree(lines, tok.to_string()))
                    }
                    _ => (),
                }
            }
            "dir" => (),
            _ => n.insert(File(
                itok.next().unwrap().to_string(),
                tok.parse::<i32>().unwrap(),
            )),
        }
    }
    n
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.clone()
        .into_iter()
        .filter(|n| matches!(n, Dir(_, _, _)))
        .map(|n| -> i32 { n.get_size() })
        .filter(|&sz| sz <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let fs_size = 70_000_000;

    let req = 30_000_000 - (fs_size - data.get_size());

    data.clone()
        .into_iter()
        .filter(|n| matches!(n, Dir(_, _, _)))
        .map(|n| -> i32 { n.get_size() })
        .filter(|&sz| sz >= req)
        .min()
        .unwrap()
}
