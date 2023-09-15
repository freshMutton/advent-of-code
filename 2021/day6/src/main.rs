use std::io::{self,BufRead};
use std::collections::HashMap;

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(parse)
        .collect::<Vec<Vec<Timer>>>();

    first_solution(&input.first().unwrap());
    second_solution(&input.first().unwrap());
}

fn parse(line: Result<String, std::io::Error>) -> Vec<Timer> {
    line
        .unwrap()
        .split(',')
        .map(|x| Timer::new(x.parse::<usize>().unwrap()))
        .collect::<_>()
}

#[derive(Clone, Copy)]
struct Timer {
    count: usize
}

impl Timer {
    fn new(count: usize) -> Self {
        Timer { count }
    }

    fn tick(self) -> Vec<Self> {
        if self.count == 0 {
            vec![Timer::new(6), Timer::new(8)]
        } else {
            vec![Timer::new(self.count - 1)]
        }
    }
}

struct World {
    fish: Vec<Timer>
}

impl World {
    fn new(fish: Vec<Timer>) -> Self {
        World { fish }
    }

    fn tick(self) -> Self {
        World { fish: self.fish.into_iter().flat_map(|fish| fish.tick()).collect::<_>() }
    }
}

#[derive(Debug)]
struct SmarterWorld {
    fish: HashMap<usize, usize>
}

impl SmarterWorld {
    fn new(fish: Vec<Timer>) -> Self {
        let mut fishMap = HashMap::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]);

        for x in fish.iter() {
            fishMap.entry(x.count).and_modify(|day| *day += 1);
        }

        SmarterWorld { fish: fishMap }
    }

    fn tick(self) -> Self {
        let mut updatedFish = SmarterWorld::new(vec![]);

        for x in 1..=8 {
                updatedFish.fish.entry(x - 1).and_modify(|day| { *day = 0 + self.fish.get(&x).unwrap() });
        }

        if let Some(existingFish) = self.fish.get(&0) {
            updatedFish.fish.entry(6).and_modify(|day| { *day += existingFish });
            updatedFish.fish.entry(8).and_modify(|day| { *day = 0 + existingFish });
        }

        updatedFish
    }
}

fn first_solution(input: &Vec<Timer>) {
    let mut world = World::new(input.to_vec());

    for _ in 1..=80 {
        world = world.tick()
    }

    println!("{:?}", world.fish.len());
}

fn second_solution(input: &Vec<Timer>) {
    let mut world = SmarterWorld::new(input.to_vec());

    for _ in 1..=256 {
        world = world.tick();
    }

    println!("{:?}", world.fish.values().sum::<usize>());
}
