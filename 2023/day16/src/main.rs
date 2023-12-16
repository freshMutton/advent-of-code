use std::{
    collections::HashSet,
    io::{self, BufRead},
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
    input
        .iter()
        .map(|line| line.as_bytes().iter().map(|c| char::from(*c)).collect())
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(usize, usize);

impl Pos {
    pub fn step(&self, dir: &Dir) -> Option<Self> {
        match *dir {
            Dir::North if self.1 != 0 => Some(Self(self.0, self.1 - 1)),
            Dir::North => None,
            Dir::South => Some(Self(self.0, self.1 + 1)),
            Dir::East => Some(Self(self.0 + 1, self.1)),
            Dir::West if self.0 != 0 => Some(Self(self.0 - 1, self.1)),
            Dir::West => None,
        }
    }
}

fn direction(c: &char, dir: &Dir) -> Vec<Dir> {
    match (c, dir) {
        ('.', _) => vec![dir.to_owned()],
        ('|', Dir::North | Dir::South) => vec![dir.to_owned()],
        ('|', _) => vec![Dir::North, Dir::South],
        ('-', Dir::East | Dir::West) => vec![dir.to_owned()],
        ('-', _) => vec![Dir::East, Dir::West],
        ('/', Dir::North) => vec![Dir::East],
        ('/', Dir::South) => vec![Dir::West],
        ('/', Dir::East) => vec![Dir::North],
        ('/', Dir::West) => vec![Dir::South],
        ('\\', Dir::North) => vec![Dir::West],
        ('\\', Dir::South) => vec![Dir::East],
        ('\\', Dir::East) => vec![Dir::South],
        ('\\', Dir::West) => vec![Dir::North],
        _ => panic!("shit's fucked"),
    }
}

fn illuminate(map: &Vec<Vec<char>>, start: (Pos, Dir)) -> HashSet<Pos> {
    let mut path = HashSet::new();
    let mut seen = HashSet::new();

    let mut queue = vec![start];

    while queue.len() > 0 {
        let (pos, dir) = queue.pop().unwrap();

        if path.contains(&(pos.to_owned(), dir.to_owned())) {
            continue;
        }

        for new_dir in direction(&map[pos.1][pos.0], &dir).iter() {
            if let Some(new_pos) = pos.step(new_dir) {
                if new_pos.0 < map[0].len() && new_pos.1 < map.len() {
                    queue.push((new_pos, new_dir.to_owned()));
                }
            }
        }

        path.insert((pos.to_owned(), dir.to_owned()));
        seen.insert(pos.to_owned());
    }

    seen
}

fn first_solution(input: &Vec<String>) -> String {
    let result = illuminate(&parse(input), (Pos(0, 0), Dir::East)).len();

    format!("{:?}", result)
}

fn second_solution(input: &Vec<String>) -> String {
    let map = parse(input);

    let mut max_count = usize::MIN;

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    for y in 0..=max_y {
        let a = illuminate(&map, (Pos(0, y), Dir::East)).len();
        let b = illuminate(&map, (Pos(max_x, y), Dir::West)).len();

        max_count = max_count.max(a.max(b));
    }

    for x in 0..=max_x {
        let a = illuminate(&map, (Pos(x, 0), Dir::South)).len();
        let b = illuminate(&map, (Pos(x, max_y), Dir::North)).len();

        max_count = max_count.max(a.max(b));
    }

    format!("{:?}", max_count)
}
