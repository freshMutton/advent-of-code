use std::collections::{HashMap, HashSet};
use std::io::{self,BufRead};

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
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let max_x = input[0].len();
    let max_y = input.len();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                ys.push(Vec::new());
                xs.push(Vec::new());
            }

            let height = c.to_digit(10).unwrap();


            ys[x].push(height);
            xs[y].push(height);
        }
    }

    for x in 0..max_x {
        for y in 0..max_y {
            if x == 0 || y == 0 || x + 1 == max_x || y + 1 == max_y {
                visible.insert((x, y));
            } else {
                let height = xs[x][y];

                let visible_north = &ys[y][0..x].iter().all(|x| *x < height);
                let visible_south = &ys[y][x+1..max_x].iter().all(|x| *x < height);
                let visible_east = &xs[x][0..y].iter().all(|x| *x < height);
                let visible_west = &xs[x][y+1..max_y].iter().all(|x| *x < height);

                if *visible_north ||
                *visible_south ||
                *visible_east ||
                *visible_west {
                    visible.insert((x, y));
                }
            }
        }
    }

    format!("{}", visible.len())
}

fn second_solution(input: &Vec<String>) -> String {
    let mut highest_score = 0;

    let max_x = input[0].len();
    let max_y = input.len();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                ys.push(Vec::new());
                xs.push(Vec::new());
            }

            let height = c.to_digit(10).unwrap();


            ys[x].push(height);
            xs[y].push(height);
        }
    }

    for x in 0..max_x {
        for y in 0..max_y {
            if x == 0 || y == 0 || x + 1 == max_x || y + 1 == max_y {
                continue;
            } else {
                let height = xs[x][y];

                let north = &ys[y][0..x].iter().rev().fold(
                    (0, false),
                    |(acc, skip), x| {
                        if skip {
                            (acc, skip)
                        } else {
                            (acc + 1, *x >= height)
                        }
                    }
                );

                let south = &ys[y][x+1..max_x].iter().fold(
                    (0, false),
                    |(acc, skip), x| {
                        if skip {
                            (acc, skip)
                        } else {
                            (acc + 1, *x >= height)
                        }
                    }
                );

                let east = &xs[x][0..y].iter().rev().fold(
                    (0, false),
                    |(acc, skip), x| {
                        if skip {
                            (acc, skip)
                        } else {
                            (acc + 1, *x >= height)
                        }
                    }
                );

                let west = &xs[x][y+1..max_y].iter().fold(
                    (0, false),
                    |(acc, skip), x| {
                        if skip {
                            (acc, skip)
                        } else {
                            (acc + 1, *x >= height)
                        }
                    }
                );

                let score = north.0 * south.0 * east.0 * west.0;

                if score > highest_score {
                    highest_score = score;
                }
            }
        }
    }

    format!("{}", highest_score)
}
