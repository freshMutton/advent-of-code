use std::io::{self,BufRead};
use std::collections::HashMap;

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(parse)
        .collect::<Vec<Solution>>();

    first_solution(&input);
    //second_solution(&input.first().unwrap());
}

fn parse(input: Result<String, std::io::Error>) -> Solution {
    let input_str = input.unwrap();
    let mut separated = input_str.split(" | ");

    let parsed_input = separated
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let parsed_output = separated
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    Solution { input: parsed_input, output: parsed_output }
}

struct Solution {
    input: Vec<String>,
    output: Vec<String>
}

fn first_solution(input: &Vec<Solution>) {
    let mut result : HashMap<usize, usize> = HashMap::new();

    for solution in input.into_iter() {
        for output in solution.output.iter() {
            println!("{:#?}", output);
            match output.len() {
                2 => { result.entry(1).and_modify(|x| *x += 1).or_insert(1); },
                3 => { result.entry(7).and_modify(|x| *x += 1).or_insert(1); },
                4 => { result.entry(4).and_modify(|x| *x += 1).or_insert(1); },
                7 => { result.entry(8).and_modify(|x| *x += 1).or_insert(1); },
                _ => {}
            };
        }
    }

    println!("{:#?}", result.values().sum::<usize>());
}
