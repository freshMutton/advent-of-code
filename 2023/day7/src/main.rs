use std::collections::HashMap;
use std::io::{self, BufRead};

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
struct Hand {
    cards: HashMap<char, usize>,
    hand: Vec<char>,
    bid: usize,
    has_jokers: bool,
}

impl Hand {
    pub fn parse(input: &String) -> Self {
        let mut parts = input.split_whitespace();

        let hand = parts.next().unwrap().chars().collect::<Vec<char>>();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();

        let mut cards = HashMap::new();

        for card in hand.iter() {
            cards
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        Self {
            cards,
            hand,
            bid,
            has_jokers: false,
        }
    }

    pub fn handle_jokers(mut self) -> Self {
        // handle all jokers
        if *self.cards.get(&'J').unwrap_or(&0) == 5 {
            self.has_jokers = true;
        } else {
            // if there's jokers, remove them from the cards hashmap, get the count
            let jokers_count = self.cards.remove(&'J').unwrap_or(0);

            if jokers_count > 0 {
                self.has_jokers = true;
            }

            let mut largest = (' ', usize::MIN);

            for card in self.cards.iter() {
                if card.1 > &largest.1 {
                    largest = (*card.0, *card.1);
                }
            }

            // add the count of jokers to the most frequent card in the hand
            self.cards
                .entry(largest.0)
                .and_modify(|count| *count += jokers_count);
        }

        self
    }

    fn value(card: &char) -> usize {
        if card.is_numeric() {
            card.to_digit(10).unwrap().try_into().unwrap()
        } else {
            match card {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("invalid input"),
            }
        }
    }

    fn card_value(&self, pos: usize) -> usize {
        let value = Hand::value(&self.hand[pos]);

        if self.has_jokers && value == 11 {
            1
        } else {
            value
        }
    }

    pub fn sort(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.rank();
        let b = other.rank();

        if a == b {
            let mut order = std::cmp::Ordering::Equal;

            let mut i = 0;
            while order == std::cmp::Ordering::Equal && i < self.hand.len() {
                let a = self.card_value(i);
                let b = other.card_value(i);

                order = a.partial_cmp(&b).unwrap();

                i += 1;
            }

            order
        } else if a > b {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }

    pub fn rank(&self) -> usize {
        // five of a kind
        if self.cards.len() == 1 {
            return 7;
        }

        if self.cards.len() == 2 {
            let mut hand = self.cards.values();

            let first = hand.next().unwrap();
            let last = hand.next().unwrap();

            // four of a kind
            if *first == 4 || *last == 4 {
                return 6;
            } else {
                // full house
                return 5;
            }
        }

        if self.cards.len() == 3 {
            let mut hand = self.cards.values();

            let first = hand.next().unwrap();
            let middle = hand.next().unwrap();
            let last = hand.next().unwrap();

            // three of a kind
            if *first == 3 || *middle == 3 || *last == 3 {
                return 4;
            } else {
                // two pair
                return 3;
            }
        }

        if self.cards.len() == 4 {
            return 2;
        }

        // high card
        if self.cards.len() == 5 {
            return 1;
        }

        return 0;
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let mut result = input
        .iter()
        .map(|line| Hand::parse(line))
        .collect::<Vec<_>>();

    result.sort_by(|a, b| a.sort(b));

    let sum = result
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();

    format!("{:#?}", sum)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut result = input
        .iter()
        .map(|line| Hand::handle_jokers(Hand::parse(line)))
        .collect::<Vec<_>>();

    result.sort_by(|a, b| a.sort(b));

    let sum = result
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();

    format!("{:#?}", sum)
}
