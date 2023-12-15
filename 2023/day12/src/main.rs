use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
    iter,
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

#[derive(Debug, Clone, PartialEq)]
enum State {
    Ok,
    Broken,
    Unknown,
}

impl State {
    fn parse(c: &char) -> Self {
        match c {
            '#' => Self::Broken,
            '.' => Self::Ok,
            '?' => Self::Unknown,
            _ => panic!("invalid"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "."),
            Self::Broken => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

fn parse(input: &String) -> (Vec<State>, Vec<usize>) {
    let mut parts = input.split_whitespace();

    let springs = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| State::parse(&c))
        .collect();
    let groups = parts
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    (springs, groups)
}

fn valid(springs: &Vec<State>, groups: &Vec<usize>) -> bool {
    let mut queue = groups.clone();
    queue.reverse();

    let mut consuming_group = false;

    for i in 0..springs.len() {
        if let Some(remainder) = queue.last() {
            if *remainder == 0 {
                if springs[i] != State::Ok {
                    return false;
                }

                queue.pop();
                consuming_group = false;
            }
        }

        if let Some(remainder) = queue.last_mut() {
            if consuming_group && springs[i - 1] != State::Broken {
                return false;
            }

            if springs[i] == State::Broken {
                *remainder -= 1;
                consuming_group = true;
            }
        } else {
            // any subsequent Broken springs are invalid
            if springs[i] == State::Broken {
                return false;
            }
        }
    }

    queue.iter().sum::<usize>() == 0
}

fn generate_permutations(
    (springs, groups): (Vec<State>, Vec<usize>),
) -> Vec<(Vec<State>, Vec<usize>)> {
    let product = springs.clone();

    product
        .iter()
        .enumerate()
        .filter(|(_, x)| *x == &State::Unknown)
        .map(|(i, _)| vec![(i, State::Broken), (i, State::Ok)])
        .multi_cartesian_product()
        .map(|replacements| {
            let mut updated = springs.clone();

            for (i, val) in replacements.into_iter() {
                updated[i] = val;
            }

            (updated, groups.clone())
        })
        .collect::<Vec<_>>()
}

fn first_solution(input: &Vec<String>) -> String {
    let result = input
        .iter()
        .map(parse)
        .flat_map(generate_permutations)
        .filter(|(springs, groups)| valid(&springs, &groups))
        .count();

    format!("{:?}", result)
}

fn expand((springs, groups): (Vec<State>, Vec<usize>)) -> (Vec<State>, Vec<usize>) {
    (
        iter::repeat(springs)
            .take(5)
            .fold(Vec::new(), |mut acc, mut next| {
                if acc.len() > 0 {
                    acc.push(State::Unknown);
                }

                acc.append(&mut next);

                acc
            }),
        groups.repeat(5),
    )
}

fn count_permutations((springs, groups): (Vec<State>, Vec<usize>)) -> usize {
    fn next(
        s: usize,
        g: usize,
        springs: &Vec<State>,
        groups: &Vec<usize>,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if g == groups.len() {
            if springs[s..].iter().all(|s| *s != State::Broken) {
                return 1;
            } else {
                return 0;
            }
        }

        if s == springs.len() {
            if g == groups.len() {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(value) = cache.get(&(s, g)) {
            return *value;
        }

        let spring = &springs[s];
        let group = &groups[g];

        match spring {
            State::Ok => next(s + 1, g, &springs, &groups, cache),
            State::Unknown | State::Broken => {
                let mut result = 0;

                if *spring == State::Unknown {
                    result += next(s + 1, g, &springs, &groups, cache);
                }

                if s + group > springs.len()
                    || springs[s..s + group].iter().any(|s| *s == State::Ok)
                {
                    result += 0;
                } else {
                    if s + group == springs.len() {
                        if groups.len() - g == 1 {
                            result += 1;
                        } else {
                            result += 0;
                        }
                    } else {
                        if springs[s + group] != State::Broken && s + group + 1 <= springs.len() {
                            result += next(s + group + 1, g + 1, &springs, &groups, cache);
                        } else {
                            result += 0;
                        }
                    }
                }

                cache.insert((s, g), result);
                return result;
            }
        }
    }

    next(0, 0, &springs, &groups, &mut HashMap::new())
}

fn second_solution(input: &Vec<String>) -> String {
    let result = input
        .iter()
        .map(parse)
        .map(expand)
        .map(count_permutations)
        .sum::<usize>();

    format!("{:#?}", result)
}
