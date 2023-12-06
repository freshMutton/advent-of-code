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

fn parse(input: &str) -> Vec<usize> {
    input.split_whitespace().skip(1).map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>()
}

fn first_solution(input: &Vec<String>) -> String {
    let mut input = input.iter();

    let times = parse(input.next().unwrap());
    let distance = parse(input.next().unwrap());

    let mut winning_presses = vec!();

    for (time, distance) in times.iter().zip(distance.iter()) {
        let mut won = 0;

        for pressed_ms in 1..*time {
            let run_time = time - pressed_ms;

            if run_time * pressed_ms > *distance {
                won += 1;
            }
        }

        if won > 0 {
            winning_presses.push(won);
        }
    }

    format!("{}", winning_presses.iter().product::<usize>())
}

fn parse_kerned(input: &str) -> usize {
    input.split_whitespace().skip(1).fold("".to_string(), |mut acc, char| { acc.push_str(char); acc }).parse::<usize>().unwrap()
}

fn second_solution(input: &Vec<String>) -> String {
    let mut input = input.iter();

    let time = parse_kerned(input.next().unwrap());
    let distance = parse_kerned(input.next().unwrap());

    let mut won = 0;

    for pressed_ms in 1..time {
        let run_time = time - pressed_ms;

        if run_time * pressed_ms > distance {
            won += 1;
        }
    }

    format!("{}", won)
}
