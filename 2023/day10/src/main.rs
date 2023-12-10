use std::{io::{self, BufRead}, collections::{HashMap, HashSet}};

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

fn parse(input: &Vec<String>) -> (HashMap<(usize, usize), char>, (usize, usize)) {
    let mut map = HashMap::new();

    let mut start = (0, 0);

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (j, i);
            }

            map.insert((j, i), c);
        }
    }


    let above = usize::checked_sub(start.1, 1).map(|y| map.get(&(start.0, y)).unwrap());
    let below = Some(map.get(&(start.0, start.1 + 1)).unwrap());
    let west = usize::checked_sub(start.0, 1).map(|x| map.get(&(x, start.1)).unwrap());
    let east = Some(map.get(&(start.0 + 1, start.1)).unwrap());

    *map.get_mut(&start).unwrap() = match (above, below, west, east) {
        (Some('|'), Some('|'), _, _) => '|',
        (Some('-'), Some('-'), _, _) => '-',

        (Some('7') | Some('F') | Some('|'), _, _, Some('7') | Some('J') | Some('-')) => 'L',
        (Some('7') | Some('F') | Some('|'), _, Some('F') | Some('L') | Some('-'), _) => 'J',

        (_, Some('J') | Some('L') | Some('|'), _, Some('7') | Some('J') | Some('-')) => 'F',
        (_, Some('J') | Some('L') | Some('|'), Some('F') | Some('L') | Some('-'), _) => '7',

        _ => panic!("invalid combination"),
    };

    (map, start)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn next_direction(dir: &Dir, piece: &char) -> Dir {
    match (*piece, *dir) {
        ('|', _) => *dir,
        ('-', _) => *dir,

        ('L', Dir::S) => Dir::E,
        ('L', Dir::W) => Dir::N,

        ('J', Dir::S) => Dir::W,
        ('J', Dir::E) => Dir::N,

        ('F', Dir::N) => Dir::E,
        ('F', Dir::W) => Dir::S,

        ('7', Dir::N) => Dir::W,
        ('7', Dir::E) => Dir::S,

        _ => panic!("invalid direction: {:?} {:?}", dir, piece)
    }
}

fn next_position(pos: &(usize, usize), dir: &Dir) -> (usize, usize) {
    match *dir {
        Dir::N => (pos.0, pos.1 - 1),
        Dir::S => (pos.0, pos.1 + 1),
        Dir::E => (pos.0 + 1, pos.1),
        Dir::W => (pos.0 - 1, pos.1),
    }
}

fn step(pos: &((usize, usize), Dir), piece: &char) -> ((usize, usize), Dir) {
    let direction = next_direction(&pos.1, piece);
    let position = next_position(&pos.0, &direction);

    (position, direction)
}

fn first_solution(input: &Vec<String>) -> String {
    let (map, pos) = parse(input);

    let start = map.get(&pos).unwrap();

    let (mut tortoise, mut hare) = match start {
        '|' => (step(&(pos, Dir::N), &'|'), step(&(pos, Dir::S), &'|')),
        '-' => (step(&(pos, Dir::E), &'-'), step(&(pos, Dir::W), &'-')),
        'L' => (step(&(pos, Dir::S), &'L'), step(&(pos, Dir::W), &'L')),
        'J' => (step(&(pos, Dir::S), &'J'), step(&(pos, Dir::E), &'J')),
        'F' => (step(&(pos, Dir::N), &'F'), step(&(pos, Dir::W), &'F')),
        '7' => (step(&(pos, Dir::N), &'7'), step(&(pos, Dir::E), &'7')),
        _ => panic!("invalid start char"),
    };

    let mut count = 1;

    while tortoise.0 != hare.0 {
        count += 1;

        let piece = map.get(&tortoise.0).unwrap();
        tortoise = step(&tortoise, piece);

        let piece = map.get(&hare.0).unwrap();
        hare = step(&hare, piece);
    }

    format!("{:#?}", count)
}

fn second_solution(input: &Vec<String>) -> String {
    let (map, pos) = parse(input);

    let start = map.get(&pos).unwrap();

    let (mut tortoise, mut hare) = match start {
        '|' => (step(&(pos, Dir::N), &'|'), step(&(pos, Dir::S), &'|')),
        '-' => (step(&(pos, Dir::E), &'-'), step(&(pos, Dir::W), &'-')),
        'L' => (step(&(pos, Dir::S), &'L'), step(&(pos, Dir::W), &'L')),
        'J' => (step(&(pos, Dir::S), &'J'), step(&(pos, Dir::E), &'J')),
        'F' => (step(&(pos, Dir::N), &'F'), step(&(pos, Dir::W), &'F')),
        '7' => (step(&(pos, Dir::N), &'7'), step(&(pos, Dir::E), &'7')),
        _ => panic!("invalid start char"),
    };

    let mut loop_pieces = HashSet::new();
    loop_pieces.insert(pos);
    loop_pieces.insert(tortoise.0);
    loop_pieces.insert(hare.0);

    while tortoise.0 != hare.0 {

        let piece = map.get(&tortoise.0).unwrap();
        tortoise = step(&tortoise, piece);

        let piece = map.get(&hare.0).unwrap();
        hare = step(&hare, piece);

        loop_pieces.insert(tortoise.0);
        loop_pieces.insert(hare.0);
    }

    let result = map.iter()
        .filter(|(pos, _)| !loop_pieces.contains(pos))
        .map(|(pos, _)| {
            let mut count = 0;
                
            for x in 0..pos.0 {

                if !loop_pieces.contains(&(x, pos.1)) {
                    continue;
                } else {
                    count += match map.get(&(x, pos.1)).unwrap() {
                        '|' | 'L' | 'J'=> 1,
                        _ => 0,
                    };
                }

            }

            count
        })
        .filter(|count| count % 2 != 0)
        .count();

    format!("{:#?}", result)
}
