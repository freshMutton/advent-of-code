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

fn first_solution(input: &Vec<String>) -> String {
    let mut numbers = HashMap::new();
    let mut symbols = HashSet::new();

    let mut search: Option<((isize, isize), (isize, isize), String)> = None;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            
            match c {
                _ if c.is_numeric() => {
                    if let Some(ref mut _search) = search {
                        _search.1 = (x, y);
                        _search.2.push(c);
                    } else {
                        search = Some(((x, y), (x, y), c.to_string()));
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

                    if c != '.' {
                        symbols.insert((x, y));
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

    let sum = numbers.iter()
        .filter(|x| {
            for pos in x.0.0.0-1..=x.0.1.0+1 {
                if  symbols.contains(&(pos, x.0.0.1-1)) || 
                    symbols.contains(&(pos, x.0.0.1)) || 
                    symbols.contains(&(pos, x.0.0.1+1)) 
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
    format!("{}", "")
}
