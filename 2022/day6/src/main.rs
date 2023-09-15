use std::io::{self,BufRead};
use std::collections::{HashSet,VecDeque};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

fn find_unique_position(input: &String, count: usize) -> usize {
    let mut chars = input.chars();
    let mut seen = VecDeque::with_capacity(count);
    let mut position = 0;

    for (i, c) in chars.enumerate() {
        position = i;

        if seen.len() == count {
            seen.pop_back();
        }

        seen.push_front(c);

        let unique: HashSet<char> = seen.to_owned().into_iter().collect();

        if unique.len() == count {
            break;
        }
    }

    position
}

fn first_solution(input: &Vec<String>) -> String {
    let position = find_unique_position(&input[0], 4);

    format!("{}", position + 1)
}

fn second_solution(input: &Vec<String>) -> String {
    let position = find_unique_position(&input[0], 14);

    format!("{}", position + 1)
}
