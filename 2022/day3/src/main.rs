use std::io::{self,BufRead};
use std::collections::HashSet;

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

const alphabet: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Rucksack {
    first: HashSet<usize>,
    second: HashSet<usize>,
}

impl Rucksack {
    fn new(input: &String) -> Self {
        let len = input.len();

        let first = Rucksack::parse_compartment(&input[..(len / 2)]);
        let second = Rucksack::parse_compartment(&input[(len / 2)..]);

        Rucksack { first, second }
    }

    fn parse_compartment(items: &str) -> HashSet<usize> {
        items.chars()
            .map(|x| alphabet.chars().position(|y| x == y).unwrap())
            .collect::<HashSet<usize>>()
    }

    fn find_duplicate(self) -> usize {
        let mut duplicates = self.first.intersection(&self.second);

        *duplicates.next().unwrap()
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let mut duplicates: Vec<usize> = Vec::new();

    for rucksack in input.iter() {
        duplicates.push(Rucksack::new(rucksack).find_duplicate());
    }

    format!("{}", duplicates.iter().sum::<usize>())
}

fn second_solution(input: &Vec<String>) -> String {
    let mut badges: Vec<usize> = Vec::new();

    for group in input.chunks(3) {
        let mut group = group.iter();

        let a = Rucksack::parse_compartment(group.next().unwrap());
        let b = Rucksack::parse_compartment(group.next().unwrap());
        let c = Rucksack::parse_compartment(group.next().unwrap());

        let badge = &(&a & &b) & &c;

        badges.push(*badge.iter().next().unwrap());
    }

    format!("{}", badges.iter().sum::<usize>())
}
