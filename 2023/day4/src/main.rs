use std::collections::*;
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

fn first_solution(input: &Vec<String>) -> String {
    let sum = input
        .iter()
        .map(|x| {
            let mut parts = x.split(": ");
            parts.next(); // ignore "Card x"

            let mut numbers = parts.next().unwrap().split(" | ");

            let mut winners = numbers
                .next()
                .unwrap()
                .split(' ')
                .collect::<HashSet<&str>>();
            let mut have = numbers
                .next()
                .unwrap()
                .split(' ')
                .collect::<HashSet<&str>>();

            winners.remove("");
            have.remove("");

            let count = winners.intersection(&have).count();

            if count == 0 || count == 1 {
                count
            } else {
                2_usize.pow((count - 1).try_into().unwrap())
            }
        })
        .sum::<usize>();

    format!("{:?}", sum)
}

fn second_solution(input: &Vec<String>) -> String {
    let won_cards = input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, x)| {
            let mut parts = x.split(": ");
            parts.next(); // ignore "Card x"

            let mut numbers = parts.next().unwrap().split(" | ");

            let mut winners = numbers
                .next()
                .unwrap()
                .split(' ')
                .collect::<HashSet<&str>>();
            let mut have = numbers
                .next()
                .unwrap()
                .split(' ')
                .collect::<HashSet<&str>>();

            winners.remove("");
            have.remove("");

            let matches = winners.intersection(&have).count();
            let count = *acc.entry(i).or_insert(1);

            if matches > 0 {
                let next_card = i + 1;
                for card in next_card..next_card + matches {
                    *acc.entry(card).or_insert(1) += count;
                }
            }

            acc
        })
        .values()
        .sum::<usize>();

    format!("{:?}", won_cards)
}
