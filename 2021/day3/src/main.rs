use std::io::{self, BufRead};

fn main() {
    let input : Vec<String> = io::stdin() // change
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<_>();

        first_solution(&input);
        second_solution(&input);
}

#[derive(Debug)]
struct BitStat {
    zeros: usize,
    ones: usize
}

impl BitStat {
    fn new(input: &Vec<char>) -> BitStat {
        input
            .into_iter()
            .fold(
                BitStat { zeros: 0, ones: 0 },
                |acc,x| {
                    if x == &'0' {
                        BitStat { zeros: acc.zeros + 1, ones: acc.ones }
                    } else {
                        BitStat { zeros: acc.zeros, ones: acc.ones + 1 }
                    }
                }
            )
    }

    fn get_stats(input: &Vec<String>) -> Vec<BitStat> {
        let length = input.get(0).unwrap().len();
        let mut result = Vec::with_capacity(length);

        for i in 0..length {
            result.push(BitStat::new(
                &input
                    .into_iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<Vec<char>>()
            ))
        }

        result
    }
}


#[derive(Debug)]
struct Stats {
    gamma: String,
    epsilon: String
}

impl Stats {
    fn calculate_gamma(&self) -> isize {
        isize::from_str_radix(&self.gamma, 2).unwrap()
    }

    fn calculate_epsilon(&self) -> isize {
        isize::from_str_radix(&self.epsilon, 2).unwrap()
    }

    fn calculate_solution(&self) -> isize {
        let gamma = &self.calculate_gamma();
        let epsilon = &self.calculate_epsilon();

        gamma * epsilon
    }
}

fn first_solution(input: &Vec<String>) {
    let solution = BitStat::get_stats(input)
        .into_iter()
        .fold(
            Stats { gamma: "".to_string(), epsilon: "".to_string() },
            |acc, x| {
                if x.zeros > x.ones {
                    Stats { gamma: acc.gamma + "0", epsilon: acc.epsilon + "1" }
                } else {
                    Stats { gamma: acc.gamma + "1", epsilon: acc.epsilon + "0" }
                }
            }
        );

    println!("first solution: {}", solution.calculate_solution());
}

fn second_solution(input: &Vec<String>) {
    let mut o2 = input.clone();
    let mut pos = 0;

    while &o2.len() > &1 {
        let stats = BitStat::new(
            &o2
                .clone()
                .into_iter()
                .map(|line| line.chars().nth(pos).unwrap())
                .collect::<_>()
        );

        o2.retain(|x| {
            let currentChar = x.chars().nth(pos).unwrap();

            currentChar == '0' && stats.zeros > stats.ones ||
            currentChar == '1' && stats.zeros <= stats.ones

        });

        pos += 1;
    }

    let mut co2 = input.clone();
    pos = 0;

    while &co2.len() > &1 {
        let stats = BitStat::new(
            &co2
                .clone()
                .into_iter()
                .map(|line| line.chars().nth(pos).unwrap())
                .collect::<_>()
        );

        co2.retain(|x| {
            let currentChar = x.chars().nth(pos).unwrap();

            currentChar == '1' && stats.zeros > stats.ones ||
            currentChar == '0' && stats.zeros <= stats.ones

        });

        pos += 1;
    }

    let o2_result = isize::from_str_radix(&o2.pop().unwrap(), 2).unwrap();
    let co2_result = isize::from_str_radix(&co2.pop().unwrap(), 2).unwrap();

    println!("{:?}", o2_result);
    println!("{:?}", co2_result);
    println!("second solution: {:?}", o2_result * co2_result)
}
