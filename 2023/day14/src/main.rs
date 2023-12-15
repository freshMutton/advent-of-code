use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::Range,
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

fn parse(input: &Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn rotate(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![];

    for i in 0..input.len() {
        let mut line = vec![];

        for j in 0..input[0].len() {
            line.push(input[j][i]);
        }

        result.push(line);
    }

    result
}

fn tilt(input: &Vec<char>) -> Vec<char> {
    let mut result = input.to_owned();

    for i in 0..result.len() {
        if i == 0 {
            continue;
        }

        if result[i] == 'O' {
            let mut new_pos = i;

            for j in (0..i).rev() {
                if result[j] == '#' || result[j] == 'O' {
                    break;
                }

                new_pos = j;
            }

            result.swap(i, new_pos);
        }
    }

    result
}

fn tilt_gridwise(input: &Vec<Vec<char>>, direction: &str) -> Vec<Vec<char>> {
    let mut output = input.to_owned();

    fn reverse_if(range: Range<usize>, should_reverse: bool) -> Box<dyn Iterator<Item = usize>> {
        if should_reverse {
            Box::new(range.rev().into_iter())
        } else {
            Box::new(range.into_iter())
        }
    }

    for y in reverse_if(0..output.len(), direction == "south") {
        for x in reverse_if(0..output[0].len(), direction == "east") {
            if output[y][x] == 'O' {
                let mut new_x = x;
                let mut new_y = y;

                match direction {
                    "north" => {
                        for i in (0..y).rev() {
                            if output[i][x] == '#' || output[i][x] == 'O' {
                                break;
                            }

                            new_y = i;
                        }
                    }
                    "south" => {
                        for i in y + 1..output.len() {
                            if output[i][x] == '#' || output[i][x] == 'O' {
                                break;
                            }

                            new_y = i;
                        }
                    }
                    "east" => {
                        for i in x + 1..output[0].len() {
                            if output[y][i] == '#' || output[y][i] == 'O' {
                                break;
                            }

                            new_x = i;
                        }
                    }
                    "west" => {
                        for i in (0..x).rev() {
                            if output[y][i] == '#' || output[y][i] == 'O' {
                                break;
                            }

                            new_x = i;
                        }
                    }
                    _ => panic!("shit's fucked"),
                }

                let temp = output[new_y][new_x];
                output[new_y][new_x] = output[y][x];
                output[y][x] = temp;
            }
        }
    }

    output
}

fn first_solution(input: &Vec<String>) -> String {
    let result = rotate(parse(input))
        .iter()
        .map(tilt)
        .map(|rocks| {
            rocks
                .iter()
                .rev()
                .enumerate()
                .filter(|(_, r)| *r == &'O')
                .map(|(i, _)| i + 1)
                .sum::<usize>()
        })
        .sum::<usize>();

    format!("{:#?}", result)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut result = parse(input);

    let mut cache = HashMap::new();

    for i in 0..1000000000 {
        result = tilt_gridwise(&result, "north");
        result = tilt_gridwise(&result, "west");
        result = tilt_gridwise(&result, "south");
        result = tilt_gridwise(&result, "east");

        if let Some(start) = cache.get(&format!("{:?}", result)) {
            println!("cycle detected at: {}", start);
            let remaining = 1000000000 - 1 - i;
            let loop_len = i - start;

            for _ in 0..remaining % loop_len {
                result = tilt_gridwise(&result, "north");
                result = tilt_gridwise(&result, "west");
                result = tilt_gridwise(&result, "south");
                result = tilt_gridwise(&result, "east");
            }

            break;
        }

        cache.insert(format!("{:?}", result), i);
    }

    let sum = rotate(result)
        .iter()
        .map(|rocks| {
            rocks
                .iter()
                .rev()
                .enumerate()
                .filter(|(_, r)| *r == &'O')
                .map(|(i, _)| i + 1)
                .sum::<usize>()
        })
        .sum::<usize>();

    format!("{:?}", sum)
}
