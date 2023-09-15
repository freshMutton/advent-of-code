use std::io::{self, BufRead};

fn main() {
    let input : Vec<usize> = io::stdin() // change
        .lock()
        .lines()
        .map(|l| l.parse::<_>().unwrap())
        .collect::<_>();

    first_solution(&input);
    second_solution(&input);
}

struct Cursor {
    prev: Option<usize>,
    acc: usize
}

fn first_solution(input: &Vec<usize>) {
    let solution = input
        .into_iter()
        .fold(
            Cursor { prev: None, acc: 0 },
            |acc, x| {
                if let Some(prev) = acc.prev {
                    if x > &prev {
                        Cursor { prev: Some(*x), acc: acc.acc + 1 }
                    } else {
                        Cursor { prev: Some(*x), acc: acc.acc }
                    }
                } else {
                    Cursor { prev: Some(*x), acc: acc.acc }
                }
            }
        );

        println!("depth increased: {} times", solution.acc);
}

fn second_solution(input: &Vec<usize>) {
    let solution = input
        .windows(3)
        .fold(
            Cursor { prev: None, acc: 0 },
            |acc, xs| {
                let sum = xs.iter().sum::<usize>();

                if let Some(prev) = acc.prev {
                    if sum > prev {
                        Cursor { prev: Some(sum), acc: acc.acc + 1 }
                    } else {
                        Cursor { prev: Some(sum), acc: acc.acc }
                    }
                } else {
                    Cursor { prev: Some(sum), acc: acc.acc }
                }
            }
        );

    println!("windowed depth increased: {}", solution.acc);
}
