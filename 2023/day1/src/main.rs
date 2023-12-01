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
    let mut sum = 0;

    for line in input.iter() {
        let mut first = None;
        let mut last = None;

        for c in line.chars() {
            if c.is_numeric() {
                if first == None {
                    first = Some(c);
                    last = Some(c);
                } else {
                    last = Some(c);
                }
            }
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap());

        sum += number.parse::<usize>().unwrap();
    }


    format!("{}", sum)
}

fn parse_digit(search: &str) -> Option<char> {
    let digits = vec!(
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    );

    for digit in digits.iter() {
        // ugly, but cbf going for a parser-combinator
        if search.ends_with(digit.0) {
            return Some(digit.1)
        }
    }

    None
}

fn second_solution(input: &Vec<String>) -> String {
    let mut sum = 0;

    for line in input.iter() {
        let mut first = None;
        let mut last = None;
        let mut search = "".to_string();

        for c in line.chars() {
            if c.is_alphabetic() {
                search.push(c);
            }

            let mut digit = parse_digit(&search);

            if c.is_numeric() {
                digit = Some(c);
            }

            if digit != None {
                // overlapping words "count" -> "oneight" == 18
                search = c.into();
            }

            if let Some(d) = digit {
                if first == None {
                    first = Some(d);
                    last = Some(d);
                } else {
                    last = Some(d);
                }
            }
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap());
        sum += number.parse::<usize>().unwrap();
    }

    format!("{}", sum)
}
