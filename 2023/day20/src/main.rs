use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

use num::Integer;

fn main() {
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<_>().unwrap())
        .collect::<_>();

    println!("first solution: {}", first_solution(&input));
    println!("second solution: {}", second_solution(&input));
}

#[derive(Clone, Debug)]
enum Module {
    Flip(usize),
    Conj(HashMap<String, usize>),
    Bcast,
}

#[derive(Clone, Debug)]
struct Node {
    id: String,
    module: Module,
    children: Vec<String>,
}

fn parse(input: &Vec<String>) -> HashMap<String, Node> {
    let mut map = HashMap::new();
    let mut child_parents: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.iter() {
        let mut parts = line.split(" -> ");

        let mut id = parts.next().unwrap();

        let module = match &id[..1] {
            "%" => {
                id = &id[1..];
                Module::Flip(0)
            }
            "&" => {
                id = &id[1..];
                Module::Conj(HashMap::new())
            }
            _ => Module::Bcast,
        };

        let children = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for child in children.iter() {
            child_parents
                .entry(child.to_string())
                .and_modify(|parents| parents.push(id.to_string()))
                .or_insert(vec![id.to_string()]);
        }

        map.insert(
            id.to_string(),
            Node {
                id: id.to_string(),
                module,
                children,
            },
        );
    }

    for (child, parents) in child_parents.iter() {
        if let Some(node) = map.get_mut(child) {
            match node.module {
                Module::Conj(_) => {
                    node.module =
                        Module::Conj(parents.iter().map(|x| (x.to_owned(), 0)).collect::<HashMap<
                            String,
                            usize,
                        >>(
                        ));
                }
                _ => continue,
            }
        }
    }

    map
}

fn signal(
    start: String,
    focal: Option<String>,
    modules: &mut HashMap<String, Node>,
) -> (usize, usize, bool) {
    let mut queue: VecDeque<(String, String, usize)> = VecDeque::new();
    queue.push_back(("button".to_string(), start, 0));

    let (mut low, mut high, mut triggered) = (0, 0, false);

    while queue.len() > 0 {
        if let Some((from, to, signal)) = queue.pop_front() {
            if signal == 0 {
                low += 1;
            } else {
                high += 1;
            }

            if let Some(node) = modules.get_mut(&to) {
                match (&node.module, signal) {
                    (Module::Flip(0), 0) => {
                        node.module = Module::Flip(1);

                        for child in node.children.iter() {
                            queue.push_back((to.to_string(), child.to_string(), 1));
                        }
                    }
                    (Module::Flip(1), 0) => {
                        node.module = Module::Flip(0);

                        for child in node.children.iter() {
                            queue.push_back((to.to_string(), child.to_string(), 0));
                        }
                    }
                    (Module::Flip(_), _) => continue,
                    (Module::Conj(ps), _) => {
                        let mut parents = ps.clone();
                        parents
                            .entry(from)
                            .and_modify(|p| *p = signal)
                            .or_insert(signal);

                        if let Some(ref focal) = focal {
                            if to == *focal && signal == 1 {
                                triggered = true;
                            }
                        }

                        let signal = match parents.values().product::<usize>() {
                            1 => 0,
                            _ => 1,
                        };

                        node.module = Module::Conj(parents);

                        for child in node.children.iter() {
                            queue.push_back((to.clone(), child.to_string(), signal));
                        }
                    }
                    (Module::Bcast, _) => {
                        for child in node.children.iter() {
                            queue.push_back((to.to_string(), child.to_string(), signal));
                        }
                    }
                }
            }
        }
    }

    (low, high, triggered)
}

fn first_solution(input: &Vec<String>) -> String {
    let mut map = parse(input);

    let (mut lows, mut highs) = (0, 0);

    for _ in 0..1000 {
        let (low, high, _) = signal("broadcaster".to_string(), None, &mut map);

        lows += low;
        highs += high;
    }

    format!("{:#?}", lows * highs)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut map = parse(input);

    let parent = map
        .values()
        .find(|node| node.children.iter().any(|n| n == "rx"))
        .map(|node| node.id.clone())
        .unwrap();

    let mut count = 0;

    let parent_counts = match &map.get(&parent).unwrap().module {
        Module::Conj(parents) => parents.len(),
        _ => panic!(""),
    };

    let mut loop_counts = Vec::new();

    loop {
        count += 1;

        let (_, _, triggered) = signal("broadcaster".to_string(), Some(parent.clone()), &mut map);

        if triggered {
            loop_counts.push(count as i64);
        }

        if loop_counts.len() == parent_counts {
            break;
        }
    }

    format!(
        "{:#?}",
        loop_counts
            .into_iter()
            .reduce(|prev, next| prev.lcm(&next))
            .unwrap()
    )
}
