use std::io::{self,BufRead};
use std::collections::{HashMap, VecDeque};

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
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(input: &String) -> Result<Self, ()> {
        let mut parts = input.split(' ');

        if parts.next().unwrap() != "move" {
            return Err(());
        }

        let count = parts.next().unwrap().parse::<usize>().unwrap();

        // "from"
        parts.next();

        let from = parts.next().unwrap().parse::<usize>().unwrap();

        // "to"
        parts.next();

        let to = parts.next().unwrap().parse::<usize>().unwrap();

        Ok(Instruction { count, from, to })
    }
}

#[derive(Debug)]
struct Stack(usize, VecDeque<char>);

impl Stack {
    fn parse(input: &String) -> Result<Vec<Self>, ()> {
        let mut cs = input.chars();
        let mut stack = Vec::new();
        let mut index = 1;

        while let Some(c) = cs.next() {
            if c == ' ' {
                // if space, should be all spaces
                if cs.next().unwrap() != ' ' {
                    return Err(());
                }

                cs.next();
            } else if c == '[' {
                // if [, should be followed by an alpha char and another ]
                let char = cs.next().unwrap();
                stack.push(Stack(index, VecDeque::from([char])));

                cs.next();
            } else {
                return Err(());
            }

            // skip the separating space, and move to next block
            cs.next();
            index += 1;
        }

        Ok(stack)
    }
}

#[derive(Debug)]
enum Parsed {
    Stack(Vec<Stack>),
    Instruction(Instruction),
    Other,
}

#[derive(Debug)]
struct Cargo {
    stacks: HashMap<usize, Stack>,
    instructions: Vec<Instruction>,
    current_instruction: usize,
}

impl Cargo {
    fn new(input: &Vec<String>) -> Self {
        let mut cargo = Cargo {
            stacks: HashMap::new(),
            instructions: Vec::new(),
            current_instruction: 0
        };

        for line in input.iter() {
            let parsed = Self::parse_line(line);

            if let Parsed::Stack(stacks) = parsed {
                for mut stack in stacks {
                    cargo.stacks
                        .entry(stack.0)
                        .and_modify(|existing| existing.1.append(&mut stack.1))
                        .or_insert(stack);
                }
            } else if let Parsed::Instruction(x) = parsed {
                cargo.instructions.push(x);
            }
        }

        cargo
    }

    fn parse_line(input: &String) -> Parsed {
        if let Ok(stack) = Stack::parse(input) {
            return Parsed::Stack(stack);
        } else if let Ok(instruction) = Instruction::parse(input) {
            return Parsed::Instruction(instruction);
        } else {
            return Parsed::Other;
        }
    }

    fn move_single(&mut self) {
        let Instruction { count, from, to } = self.instructions[self.current_instruction];

        let mut to_move = Vec::new();

        if let Some(from) = self.stacks.get_mut(&from) {
            for _ in 1..=count {
                to_move.push(from.1.pop_front().unwrap());
            }
        }

        if let Some(to) = self.stacks.get_mut(&to) {
            for x in to_move.iter() {
                to.1.push_front(*x);
            }
        }

        self.current_instruction += 1;
    }

    fn move_multiple(&mut self) {
        let Instruction { count, from, to } = self.instructions[self.current_instruction];

        let mut to_move;

        if let Some(from) = self.stacks.get_mut(&from) {

            let remainder = from.1.split_off(count);
            to_move = from.1.clone();
            from.1 = remainder;
        } else {
            panic!("could not get from stack");
        }

        if let Some(to) = self.stacks.get_mut(&to) {
            to_move.append(&mut to.1);
            to.1 = to_move;
        }

        self.current_instruction += 1;
    }

    fn read_top(&mut self) -> String{
        let mut top: Vec<(usize, char)> = self.stacks.iter()
            .map(|x| (x.0.to_owned(), x.1.1.front().unwrap().to_owned()))
            .collect();


        top.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        top.iter()
            .map(|(i, x)| x)
            .collect::<String>()
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let mut cargo = Cargo::new(input);

    while cargo.current_instruction < cargo.instructions.len() {
        cargo.move_single();
    }

    cargo.read_top()
}

fn second_solution(input: &Vec<String>) -> String {
    let mut cargo = Cargo::new(input);

    while cargo.current_instruction < cargo.instructions.len() {
        cargo.move_multiple();
    }

    cargo.read_top()
}
