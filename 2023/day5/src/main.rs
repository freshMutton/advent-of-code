use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::*;

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

#[derive(Debug)]
struct Mapping {
    destination_start: usize,
    source_start: usize,
    range: usize,
}

impl Mapping {
    pub fn parse(mut input: Vec<usize>) -> Self {
        Self {
            range: input.pop().unwrap(),
            source_start: input.pop().unwrap(),
            destination_start: input.pop().unwrap(),
        }
    }

    pub fn map(&self, input: &usize) -> Option<usize> {
        let distance = usize::checked_sub(*input, self.source_start);

        if let Some(distance) = distance {
            if distance < self.range {
                Some(self.destination_start + distance)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    mappings: HashMap<String, Vec<Mapping>>,
}

impl Almanac {
    pub fn parse(input: &Vec<String>) -> Self {
        let mut lines = input.iter();

        // seeds: 79 14 55 13
        let seeds = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // empty line
        lines.next();

        let MAPPING = [
            "seed_soil",
            "soil_fertiliser",
            "fertiliser_water",
            "water_light",
            "light_temperature",
            "temperature_humidity",
            "humidity_location",
        ];

        let mut mappings = HashMap::new();

        for key in MAPPING.iter() {
            mappings.insert(key.to_string(), Almanac::parse_mapping(&mut lines));
        }

        Self { seeds, mappings }
    }

    fn parse_mapping(lines: &mut std::slice::Iter<'_, String>) -> Vec<Mapping> {
        // seed-to-soil map:
        lines.next();
        //50 98 2
        lines
            .take_while(|line| line != &&"".to_string())
            .map(|line| {
                Mapping::parse(
                    line.split_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn map(&self, key: &str, value: &usize) -> usize {
        let mapping = self
            .mappings
            .get(key)
            .unwrap()
            .iter()
            .find_map(|x| x.map(value));

        if let Some(mapping) = mapping {
            mapping
        } else {
            *value
        }
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let almanac = Almanac::parse(input);

    let result = almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac.map(
                "humidity_location",
                &almanac.map(
                    "temperature_humidity",
                    &almanac.map(
                        "light_temperature",
                        &almanac.map(
                            "water_light",
                            &almanac.map(
                                "fertiliser_water",
                                &almanac.map("soil_fertiliser", &almanac.map("seed_soil", seed)),
                            ),
                        ),
                    ),
                ),
            )
        })
        .min();

    format!("{:?}", result)
}

// TODO: don't boil the ocean
fn second_solution(input: &Vec<String>) -> String {
    let almanac = Almanac::parse(input);

    let smallest = almanac.seeds.chunks(2).fold(usize::MAX, |smallest, range| {
        let first = *range.first().unwrap();
        let count = *range.last().unwrap();

        let new_smallest = (first..first + count)
            .par_bridge()
            .fold(
                || usize::MAX,
                |smallest, seed| {
                    let value = almanac.map(
                        "humidity_location",
                        &almanac.map(
                            "temperature_humidity",
                            &almanac.map(
                                "light_temperature",
                                &almanac.map(
                                    "water_light",
                                    &almanac.map(
                                        "fertiliser_water",
                                        &almanac.map(
                                            "soil_fertiliser",
                                            &almanac.map("seed_soil", &seed),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    );

                    if value < smallest {
                        value
                    } else {
                        smallest
                    }
                },
            )
            .min()
            .unwrap();

        if new_smallest < smallest {
            new_smallest
        } else {
            smallest
        }
    });

    format!("{:?}", smallest)
}
