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

#[derive(Debug)]
struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl Game {
    pub fn parse(input: &str) -> Self {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let parts = input.split(": ").collect::<Vec<&str>>();

        let id = parts
            .first()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in parts.last().expect("valid input").split("; ") {
            for cube in set.split(", ") {
                let cube_parts = cube.split(' ').collect::<Vec<&str>>();

                let count = cube_parts.first().expect("valid input").parse::<usize>().unwrap(); 
                let colour = cube_parts.last().expect("valid input");

                if colour == &"red" && count > max_red {
                    max_red = count;
                }

                if colour == &"green" && count > max_green {
                    max_green = count;
                }

                if colour == &"blue" && count > max_blue {
                    max_blue = count;
                }
            }
        }

        Game {
            id,
            max_red,
            max_green,
            max_blue,
        }
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let sum = input
        .iter()
        .map(|line| Game::parse(line))
        .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
        .map(|game| game.id)
        .sum::<usize>();

    format!("{}", sum)
}

fn second_solution(input: &Vec<String>) -> String {
    let sum = input
        .iter()
        .map(|line| Game::parse(line))
        .map(|game| game.max_red * game.max_green * game.max_blue)
        .sum::<usize>();

    format!("{}", sum)
}
