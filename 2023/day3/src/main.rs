use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

#[derive(Hash, Eq, PartialEq)]
struct Position(isize, isize);

fn parse(input: &Vec<String>, is_symbol: fn(char) -> bool) -> (HashMap<(Position, Position), usize>, HashSet<Position>) {
    let mut numbers : HashMap<(Position, Position), usize> = HashMap::new();
    let mut symbols : HashSet<Position> = HashSet::new();

    let mut search: Option<(Position, Position, String)> = None;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            
            match c {
                _ if c.is_numeric() => {
                    if let Some(ref mut _search) = search {
                        _search.1 = Position(x, y);
                        _search.2.push(c);
                    } else {
                        search = Some((Position(x, y), Position(x, y), c.to_string()));
                    }
                },
                _ => {
                    if let Some(_search) = search {
                        numbers.insert(
                            (_search.0, _search.1),
                            _search.2.parse::<usize>().unwrap()
                        );

                        search = None;
                    }

                    if is_symbol(c) {
                        symbols.insert(Position(x, y));
                    }
                },
            }
        }

        if let Some(_search) = search {
            numbers.insert(
                (_search.0, _search.1),
                _search.2.parse::<usize>().unwrap()
            );

            search = None;
        }
    }

    (numbers, symbols)
}

fn first_solution(input: &Vec<String>) -> String {
    let (numbers, symbols) = parse(input, |c| c != '.');

    let sum = numbers.iter()
        .filter(|x| {
            for pos in x.0.0.0-1..=x.0.1.0+1 {
                if  symbols.contains(&Position(pos, x.0.0.1-1)) || 
                    symbols.contains(&Position(pos, x.0.0.1)) || 
                    symbols.contains(&Position(pos, x.0.0.1+1)) 
                {
                    return true
                }
            }

            false
        })
        .map(|x| x.1)
        .sum::<usize>();

    format!("{}", sum)
}

fn second_solution(input: &Vec<String>) -> String {
    let (numbers, symbols) = parse(input, |c| c == '*');

    let sum = numbers
        .iter()
        .fold(HashMap::new(), |mut acc, number| {
            for x in number.0.0.0-1..=number.0.1.0+1 {
                for y in number.0.0.1-1..=number.0.0.1+1 {
                    let pos = Position(x, y);

                    if symbols.contains(&pos) {
                        acc.entry(pos)
                            .and_modify(|x: &mut Vec<usize>| x.push(*number.1))
                            .or_insert(vec!(*number.1));
                    }
                }
            }

            acc
        })
        .values()
        .filter_map(|cogs| {
            if cogs.len() == 2 {
                return Some(cogs.iter().product::<usize>());
            }

            None
        })
        .sum::<usize>();



    format!("{}", sum)
}
