use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

fn hash(chars: &[u8]) -> i64 {
    chars
        .iter()
        .fold(0, |acc, next| (acc + i64::from(*next)) * 17 % 256)
}

fn first_solution(input: &Vec<String>) -> String {
    let result = input
        .iter()
        .flat_map(|line| line.split(',').map(|s| s.as_bytes()))
        .map(hash)
        .sum::<i64>();

    format!("{:?}", result)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut result: HashMap<i64, Vec<(&str, i64)>> = HashMap::new();

    for chars in input.iter().flat_map(|line| line.split(',')) {
        let mut parts = chars.split('=').collect::<Vec<_>>();
        let mut method = '=';

        if parts.len() < 2 {
            parts = chars.split('-').collect::<Vec<_>>();
            method = '-';
        }

        let id = parts[0];
        let focal_length = parts[1].parse::<i64>();

        let hash = hash(id.as_bytes());

        if let Some(boxen) = result.get_mut(&hash) {
            if method == '-' {
                for i in 0..boxen.len() {
                    if boxen[i].0 == id {
                        boxen.remove(i);
                        break;
                    }
                }
            } else if method == '=' {
                let fl = focal_length.unwrap();

                let mut found = false;

                for i in 0..boxen.len() {
                    if boxen[i].0 == id {
                        boxen[i].1 = fl;
                        found = true;
                        break;
                    }
                }

                if !found {
                    boxen.push((id, fl));
                }
            }
        } else {
            if method == '=' {
                result.insert(hash, vec![(id, focal_length.unwrap().clone())]);
            }
        }
    }

    let sum = result.iter().fold(0, |acc, (id, lenses)| {
        acc + lenses.iter().enumerate().fold(0, |acc, (pos, (_, lens))| {
            acc + ((id + 1) * (i64::try_from(pos).unwrap() + 1) * lens)
        })
    });

    format!("{:?}", sum)
}
