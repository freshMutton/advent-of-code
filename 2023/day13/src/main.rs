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

fn chunks(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut result = vec![];

    let mut temp = vec![];

    for line in input {
        if line != "" {
            temp.push(line.to_owned());
        } else {
            result.push(temp.to_owned());
            temp.clear();
        }
    }

    result.push(temp);

    result
}

fn parse(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let horizontal = input.to_owned();
    let mut vertical = vec![];

    for x in 0..input[0].len() {
        let mut string = "".to_owned();

        for y in 0..input.len() {
            string.push_str(&input[y][x..=x]);
        }

        vertical.push(string);
    }

    (horizontal, vertical)
}

fn find_balance(input: &Vec<String>) -> Option<usize> {
    let mut balance = None;

    for i in 0..input.len() {
        if i + 1 == input.len() {
            break;
        }

        let mut left = i;
        let mut right = i + 1;

        if input[left] == input[right] {
            balance = Some(i + 1);
        }

        loop {
            if let Some(l) = left.checked_sub(1) {
                left = l;
            } else {
                if balance != None {
                    return balance;
                }
                break;
            }

            right += 1;

            if right == input.len() {
                if balance != None {
                    return balance;
                }
                break;
            }

            if input[left] != input[right] {
                balance = None;
                break;
            }
        }
    }

    balance
}

fn first_solution(input: &Vec<String>) -> String {
    let result = chunks(input)
        .iter()
        .map(parse)
        .map(|(horiz, vert)| {
            (find_balance(&horiz).unwrap_or(0) * 100) + (find_balance(&vert).unwrap_or(0))
        })
        .sum::<usize>();

    format!("{:#?}", result)
}

fn find_smudged_balance(input: &Vec<String>) -> Option<usize> {
    let balance = None;

    for i in 0..input.len() {
        let mut smudged = true;
        let mut candidate = None;

        if i + 1 == input.len() {
            break;
        }

        let mut left = i;
        let mut right = i + 1;

        let mismatches = input[left]
            .chars()
            .zip(input[right].chars())
            .map(|(l, r)| match l == r {
                false => 1,
                true => 0,
            })
            .sum::<usize>();

        if mismatches == 0 || mismatches == 1 && smudged {
            if mismatches == 1 {
                smudged = false;
            }

            candidate = Some(i + 1);

            loop {
                if let Some(l) = left.checked_sub(1) {
                    left = l;
                } else {
                    if !smudged {
                        return candidate;
                    }
                    break;
                }

                right += 1;

                if right == input.len() {
                    if !smudged {
                        return candidate;
                    }
                    break;
                }

                let mismatches = input[left]
                    .chars()
                    .zip(input[right].chars())
                    .map(|(l, r)| match l == r {
                        false => 1,
                        true => 0,
                    })
                    .sum::<usize>();

                if mismatches > 1 {
                    candidate = None;
                    break;
                }

                if mismatches == 1 && smudged {
                    smudged = false;
                }
            }
        }
    }

    balance
}

fn second_solution(input: &Vec<String>) -> String {
    let result = chunks(input)
        .iter()
        .map(parse)
        .map(|(horiz, vert)| {
            (find_smudged_balance(&horiz).unwrap_or(0) * 100)
                + (find_smudged_balance(&vert).unwrap_or(0))
        })
        .sum::<usize>();

    format!("{:?}", result)
}
