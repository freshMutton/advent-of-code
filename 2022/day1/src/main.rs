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

fn sum_elves(input: &Vec<String>) -> Vec<i32> {
    let mut sums = vec![0];
    let mut curr_idx = 0;

    for x in input {
        if x == "" {
            sums.push(0);
            curr_idx += 1;
        } else {
            sums[curr_idx] += x.parse::<_>().unwrap()
        }
    }

    sums.sort();

    sums
}

fn first_solution(input: &Vec<String>) -> String {
    let mut sums = sum_elves(input);

    sums.pop().unwrap().to_string()
}

fn second_solution(input: &Vec<String>) -> String {
    let sums = sum_elves(input);

    sums
        .iter()
        .rev()
        .take(3)
        .sum::<_>()
        .to_string()
}
