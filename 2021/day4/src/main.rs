use std::io::{self, BufRead};
use itertools::Itertools;

fn main() {
    let mut input : Vec<String> = io::stdin() // change
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<_>();

    println!("{:?}", parse_input(&mut input));

//        first_solution(&input);
//        second_solution(&input);
}

fn parse_input(input: &mut Vec<String>) -> GameState {
    let rest = input.split_off(1);

    // take the first line, this is the numbers
    let numbers : Vec<usize> = input
        .first()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap()).collect::<_>();

    let boards = rest
        .into_iter()
        .filter(|line| !line.is_empty())
        .chunks(5)
        .into_iter()
        .map(|board| Board::new(&board.collect::<Vec<String>>()))
        .collect::<Vec<Board>>();

    GameState { numbers, boards }
}

#[derive(Debug)]
struct Board {
    complete: bool,
    numbers: Vec<Vec<(usize, bool)>>
}

impl Board {
    fn new(input: &Vec<String>) -> Self {
        let numbers = input
            .iter()
            .map(|line| line.split(' ').filter(|l| !l.is_empty()).map(|number| (number.parse::<usize>().unwrap(), false)).collect::<_>())
            .collect::<Vec<Vec<(usize, bool)>>>();

        Board {
            complete: false,
            numbers
        }
    }
}

#[derive(Debug)]
struct GameState {
    numbers: Vec<usize>,
    boards: Vec<Board>
}
