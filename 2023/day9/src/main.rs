use std::io::{self, BufRead};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

fn next_in_sequence(sequence: &Vec<isize>) -> isize {
    if sequence.len() == 0 {
        0
    } else {
        let next = sequence
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>();

        sequence.last().unwrap() + next_in_sequence(&next)
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let value = input
        .iter()
        .map(|line| {
            let readings = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            next_in_sequence(&readings)
        })
        .sum::<isize>();

    format!("{:#?}", value)
}

fn second_solution(input: &Vec<String>) -> String {
    let value = input
        .iter()
        .map(|line| {
            let readings = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .rev()
                .collect::<Vec<_>>();

            next_in_sequence(&readings)
        })
        .sum::<isize>();

    format!("{:#?}", value)
}
