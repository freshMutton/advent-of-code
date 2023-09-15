use std::io::{self, BufRead};

#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
    Invalid
}

fn parse_command(unparsed: Result<String, std::io::Error>) -> Command {
    let line = unparsed.unwrap();
    let mut parts = line.split(" ");
    let command = parts.next().unwrap();
    let distance = parts.next().unwrap().parse::<usize>().unwrap();

    match command {
        "forward" => Command::Forward(distance),
        "up" => Command::Up(distance),
        "down" => Command::Down(distance),
        _ => Command::Invalid
    }
}

fn main() {
    let input : Vec<Command> = io::stdin() // change
        .lock()
        .lines()
        .map(parse_command)
        .collect::<_>();

        first_solution(&input);
        second_solution(&input);
}

struct Position {
    distance: usize,
    depth: usize
}

fn first_solution(input: &Vec<Command>) {
    let solution = input
        .into_iter()
        .fold(
            Position { distance: 0, depth: 0 },
            |acc,x| {
                match x {
                    Command::Forward(dist) => Position {
                        distance: acc.distance + dist,
                        depth: acc.depth
                    },
                    Command::Up(dist) => Position {
                        distance: acc.distance,
                        depth: acc.depth - dist
                    },
                    Command::Down(dist) => Position {
                        distance: acc.distance,
                        depth: acc.depth + dist
                    },
                    Command::Invalid => acc
                }
            }
        );

    println!("result: {}", solution.distance * solution.depth)
}

struct AimedPosition {
    aim: usize,
    distance: usize,
    depth: usize
}

fn second_solution(input: &Vec<Command>) {
    let solution = input
        .into_iter()
        .fold(
            AimedPosition { aim: 0, distance: 0, depth: 0 },
            |acc, x| {
                match x {
                    Command::Forward(dist) => AimedPosition {
                        aim: acc.aim,
                        distance: acc.distance + dist,
                        depth: acc.depth + (acc.aim * dist)
                    },
                    Command::Up(dist) => AimedPosition {
                        aim: acc.aim - dist,
                        distance: acc.distance,
                        depth: acc.depth
                    },
                    Command::Down(dist) => AimedPosition {
                        aim: acc.aim + dist,
                        distance: acc.distance,
                        depth: acc.depth
                    },
                    Command::Invalid => acc
                }
            }
        );

    println!("result: {}", solution.distance * solution.depth)
}
