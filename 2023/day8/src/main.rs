use std::{
    collections::HashMap,
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

fn parse(input: &Vec<String>) -> (Vec<char>, HashMap<&str, (&str, &str)>, &str) {
    let mut lines = input.iter();

    let steps = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next();

    let mut nodes = HashMap::new();

    for line in lines {
        let mut parts = line.split(" = (");

        let id = parts.next().unwrap();

        let mut children = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| &str[..3]);

        let left = children.next().unwrap();
        let right = children.next().unwrap();

        nodes.insert(id, (left, right));
    }

    (steps, nodes, "AAA")
}

fn first_solution(input: &Vec<String>) -> String {
    let (steps, nodes, mut current_step) = parse(input);

    let mut steps_taken = 0;

    while current_step != "ZZZ" {
        let direction = steps[steps_taken % steps.len()];
        let node = nodes.get(current_step).unwrap();

        if direction == 'L' {
            current_step = node.0;
        } else {
            current_step = node.1;
        }

        steps_taken += 1;
    }

    format!("{:?}", steps_taken)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut lines = input.iter();

    let steps = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next();

    let mut nodes = HashMap::new();
    let mut queue = vec![];

    for line in lines {
        let mut parts = line.split(" = (");

        let id = parts.next().unwrap();

        if id.ends_with('A') {
            queue.push(id);
        }

        let mut children = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| &str[..3]);

        let left = children.next().unwrap();
        let right = children.next().unwrap();

        nodes.insert(id, (left, right));
    }

    let result = queue
        .iter()
        .map(|first_step| {
            let mut current_step = first_step;
            let mut steps_taken = 0;

            while !current_step.ends_with('Z') {
                let direction = steps[steps_taken % steps.len()];
                let node = nodes.get(current_step).unwrap();

                if direction == 'L' {
                    current_step = &node.0;
                } else {
                    current_step = &node.1;
                }

                steps_taken += 1;
            }

            steps_taken
        })
        .reduce(|last, this| num::integer::lcm(last, this))
        .unwrap();

    format!("{:#?}", result)
}
