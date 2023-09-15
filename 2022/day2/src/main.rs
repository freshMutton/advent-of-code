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

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
    Invalid
}

impl Play {
    fn parse(play: &str) -> Self {
        match play {
            "A" => Play::Rock,
            "X" => Play::Rock,
            "B" => Play::Paper,
            "Y" => Play::Paper,
            "C" => Play::Scissors,
            "Z" => Play::Scissors,
            &_ =>  Play::Invalid,
        }
    }

    fn outcome(&self, outcome: &str) -> Self {
        match (self, outcome) {
            (Play::Rock, "X") => Play::Scissors,
            (Play::Rock, "Y") => Play::Rock,
            (Play::Rock, "Z") => Play::Paper,
            (Play::Rock, &_) => Play::Invalid,

            (Play::Paper, "X") => Play::Rock,
            (Play::Paper, "Y") => Play::Paper,
            (Play::Paper, "Z") => Play::Scissors,
            (Play::Paper, &_) => Play::Invalid,

            (Play::Scissors, "X") => Play::Paper,
            (Play::Scissors, "Y") => Play::Scissors,
            (Play::Scissors, "Z") => Play::Rock,
            (Play::Scissors, &_) => Play::Invalid,

            (&Play::Invalid, &_) => Play::Invalid,
        }
    }
}

#[derive(Debug)]
struct Strategy {
    challenge: Play,
    response: Play
}

impl Strategy {
    fn new_by_response(input: &String) -> Self {
        let mut plays = input.split(" ").into_iter();
        let challenge = Play::parse(plays.next().unwrap());
        let response = Play::parse(plays.next().unwrap());

        Strategy { challenge, response }
    }

    fn new_by_outcome(input: &String) -> Self {
        let mut plays = input.split(" ").into_iter();
        let challenge = Play::parse(plays.next().unwrap());
        let response = challenge.outcome(plays.next().unwrap());

        Strategy { challenge, response }
    }

    fn score(self) -> i32 {
        let rock_score = 1;
        let paper_score = 2;
        let scissors_score = 3;
        let loss_score = 0;
        let win_score = 6;
        let draw_score = 3;

        match self {
            Strategy { challenge: Play::Rock, response: Play::Rock } => draw_score + rock_score,
            Strategy { challenge: Play::Rock, response: Play::Paper } => win_score + paper_score,
            Strategy { challenge: Play::Rock, response: Play::Scissors } => loss_score + scissors_score,

            Strategy { challenge: Play::Paper, response: Play::Rock } => loss_score + rock_score,
            Strategy { challenge: Play::Paper, response: Play::Paper } => draw_score + paper_score,
            Strategy { challenge: Play::Paper, response: Play::Scissors } => win_score + scissors_score,

            Strategy { challenge: Play::Scissors, response: Play::Rock } => win_score + rock_score,
            Strategy { challenge: Play::Scissors, response: Play::Paper } => loss_score + paper_score,
            Strategy { challenge: Play::Scissors, response: Play::Scissors } => draw_score + scissors_score,

            _ => 0
        }
    }
}


fn first_solution(input: &Vec<String>) -> String {
    input
        .iter()
        .map(Strategy::new_by_response)
        .fold(0, |acc, x| acc + Strategy::score(x))
        .to_string()
}

fn second_solution(input: &Vec<String>) -> String {
    input
        .iter()
        .map(Strategy::new_by_outcome)
        .fold(0, |acc, x| acc + Strategy::score(x))
        .to_string()
}
