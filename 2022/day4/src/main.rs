use std::io::{self,BufRead};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

#[derive(Debug)]
struct Assignment(usize, usize);

impl Assignment {
    fn from_str(str: &str) -> Self {
        let mut split = str.split("-");
        let from = split.next().unwrap().parse::<usize>();
        let to = split.next().unwrap().parse::<usize>();

        Assignment(from.unwrap(), to.unwrap())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.0
    }
}

fn parse_pair(str: &String) -> (Assignment, Assignment) {
    let mut pair = str.split(",");

    let first = Assignment::from_str(pair.next().unwrap());
    let second = Assignment::from_str(pair.next().unwrap());

    (first, second)
}

fn first_solution(input: &Vec<String>) -> String {
    let result = input
        .iter()
        .map(parse_pair)
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count();

    format!("{}", result)
}

fn second_solution(input: &Vec<String>) -> String {
    let result = input
        .iter()
        .map(parse_pair)
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count();

    format!("{}", result)
}
