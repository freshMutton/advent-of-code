use std::io::{self,BufRead};
use std::collections::HashMap;

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| Vector::parse(line.unwrap()))
        .collect::<Vec<Vector>>();

    first_solution(&input);
    //second_solution(&input.first().unwrap());
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn parse(input: String) -> Self {
        let mut split = input.split(',').into_iter();

        Point {
            x: split.next().unwrap().parse::<usize>().unwrap(),
            y: split.next().unwrap().parse::<usize>().unwrap()
        }
    }
}

#[derive(Clone,Copy,Debug)]
struct Vector {
    a: Point,
    b: Point
}

impl Vector {
    fn parse(input: String) -> Self {
        let mut split = input.split(" -> ").into_iter();

        Vector {
            a: Point::parse(split.next().unwrap().to_string()),
            b: Point::parse(split.next().unwrap().to_string())
        }
    }


    fn is_horizontal(self) -> bool {
        self.a.x == self.b.x
    }

    fn is_vertical(self) -> bool {
        self.a.y == self.b.y
    }

    fn is_diagonal(self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn derive_points(self) -> Vec<Point> {
        let mut points = vec![];


        if self.is_horizontal() {
            let x_range;

            if self.a.x < self.b.x {
                x_range = self.a.x..=self.b.x;
            } else {
                x_range = self.b.x..=self.a.x;
            }

            for x in x_range {
                points.push(Point { x: x, self.a.y });
            }

        }

        if self.is_vertical() {
            let y_range;

            if self.a.y < self.b.y {
                y_range = self.a.y..=self.b.y;
            } else {
                y_range = self.b.y..=self.a.y;
            }

            for y in y_range {
                points.push(Point { x: self.a.x, y });
            }
        }

        if self.is_diagonal() {
        }

        if self.is_horizontal() || self.is_vertical() {

            for x in x_range {

                for y in y_range {
                    points.push(Point { x, y });
                }
            }
        }

        points
    }
}

fn first_solution(input: &Vec<Vector>) {
    let mut map : HashMap<Point, usize> = HashMap::new();

    for point in input.iter().flat_map(|x| x.derive_points()) {
        *map.entry(point).or_insert(0) += 1;
    }

    println!("{:#?}", map.values().filter(|x| x >= &&2).count());
}
