use std::io::{self,BufRead};
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .unwrap();

    println!("{}", input);
    //first_solution(&input);
    //second_solution(&input.first().unwrap());
}

struct Solution {
    value: usize,
    adjacencies: Vec<usize>
}

impl Solution {
    fn is_smallest(self) -> bool {
        self.adjacencies.into_iter().any(|x| x > self.value)
    }

    fn risk_level(self) -> usize {
        self.value + 1
    }
}
