use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    io::{self, BufRead},
    ops::Range,
};

use itertools::Itertools;

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

fn expand(input: &Vec<String>) -> Vec<String> {
    let verticals = input
        .iter()
        .fold(HashMap::new(), |mut acc, line| {
            for (i, c) in line.chars().enumerate() {
                acc.entry(i)
                    .and_modify(|line: &mut String| line.push(c))
                    .or_insert(format!("{}", c));
            }

            acc
        })
        .into_iter()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .collect::<HashMap<usize, String>>();

    input
        .iter()
        .flat_map(|line| {
            let mut new_line = line.clone();

            for i in verticals.keys().sorted().rev() {
                let (left, right) = new_line.split_at(*i);
                new_line = format!("{}{}{}", left, '.', right);
            }

            if new_line.chars().all(|c| c == '.') {
                vec![new_line.clone(), new_line.clone()]
            } else {
                vec![new_line]
            }
        })
        .collect::<Vec<_>>()
}

fn parse(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut parsed = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                parsed.push((x, y));
            }
        }
    }

    parsed
}

fn first_solution(input: &Vec<String>) -> String {
    let result = parse(&expand(input));

    let count = result
        .iter()
        .tuple_combinations()
        .map(|(left, right)| usize::abs_diff(left.0, right.0) + usize::abs_diff(left.1, right.1))
        .sum::<usize>();

    format!("{:#?}", count)
}

fn find_empty(input: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let xs = input
        .iter()
        .fold(HashMap::new(), |mut acc, line| {
            for (i, c) in line.chars().enumerate() {
                acc.entry(i)
                    .and_modify(|line: &mut String| line.push(c))
                    .or_insert(format!("{}", c));
            }

            acc
        })
        .into_iter()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .map(|(x, _)| x)
        .collect::<Vec<usize>>();

    let ys = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .map(|(y, _)| y)
        .collect::<Vec<usize>>();

    (xs, ys)
}

fn adjusted_distance(multiplier: usize, empties: &Vec<usize>, distance: Range<usize>) -> usize {
    empties.iter().filter(|x| distance.contains(x)).count() * (multiplier - 1)
}

fn second_solution(input: &Vec<String>) -> String {
    let result = parse(input);
    let (xs, ys) = find_empty(input);

    let count = result
        .iter()
        .tuple_combinations()
        .map(|(left, right)| {
            let x_range = Range {
                start: usize::min(left.0, right.0),
                end: usize::max(left.0, right.0),
            };

            let y_range = Range {
                start: usize::min(left.1, right.1),
                end: usize::max(left.1, right.1),
            };

            let x_adjust = adjusted_distance(1000000, &xs, x_range);
            let y_adjust = adjusted_distance(1000000, &ys, y_range);

            let x = usize::abs_diff(left.0, right.0);
            let y = usize::abs_diff(left.1, right.1);

            x + y + x_adjust + y_adjust
        })
        .sum::<usize>();

    format!("{:#?}", count)
}
