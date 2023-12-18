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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn step(&self, dir: &Dir, dist: isize) -> Self {
        match *dir {
            Dir::N => Self {
                y: self.y - dist,
                ..*self
            },
            Dir::S => Self {
                y: self.y + dist,
                ..*self
            },
            Dir::E => Self {
                x: self.x + dist,
                ..*self
            },
            Dir::W => Self {
                x: self.x - dist,
                ..*self
            },
        }
    }
}

fn parse_instruction(input: &String) -> (Dir, isize, String) {
    // R 6 (#70c710)
    let mut parts = input.split_whitespace();

    let dir = match parts.next().unwrap() {
        "U" => Dir::N,
        "D" => Dir::S,
        "L" => Dir::W,
        "R" => Dir::E,
        _ => panic!("shit's fucked"),
    };

    let dist = parts.next().unwrap().parse::<isize>().unwrap();

    let colour = parts.next().unwrap().to_owned();

    (dir, dist, colour)
}

fn shoelace_area(points: &Vec<Pos>) -> isize {
    let mut acc: isize = 0;

    for i in 0..points.len() {
        if i + 1 == points.len() {
            break;
        }

        let a = &points[i];
        let b = &points[i + 1];

        acc += (a.x + b.x) * (a.y - b.y);
    }

    acc.abs() / 2
}

fn first_solution(input: &Vec<String>) -> String {
    let mut start = Pos { x: 1, y: 1 };
    let mut path = Vec::new();
    let mut perimeter = 0;

    for instruction in input.iter() {
        let (dir, dist, _) = parse_instruction(instruction);
        let pos = start.step(&dir, dist);

        path.push(pos.clone());
        perimeter += dist;
        start = pos;
    }

    let area = shoelace_area(&path);

    format!("{:?}", area + (perimeter / 2) + 1)
}

fn parse_hexidecimal(input: &String) -> (Dir, isize) {
    // R 6 (#70c710)
    //       ^    ^
    // ------|---||
    //       dist dir
    let mut parts = input.split_whitespace();
    parts.next();
    parts.next();

    let hex = &parts.next().unwrap()[2..8];

    let dist = isize::from_str_radix(&hex[..5], 16).unwrap();

    let dir = match &hex[5..] {
        "3" => Dir::N,
        "1" => Dir::S,
        "2" => Dir::W,
        "0" => Dir::E,
        _ => panic!("shit's fucked"),
    };

    (dir, dist)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut start = Pos { x: 1, y: 1 };
    let mut path = Vec::new();
    let mut perimeter = 0;

    for instruction in input.iter() {
        let (dir, dist) = parse_hexidecimal(instruction);
        let pos = start.step(&dir, dist);

        path.push(pos.clone());
        perimeter += dist;
        start = pos;
    }

    let area = shoelace_area(&path);

    format!("{:?}", area + (perimeter / 2) + 1)
}
