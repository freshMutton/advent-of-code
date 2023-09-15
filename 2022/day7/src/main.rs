use std::collections::HashMap;
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
enum Filesystem {
    File(String, i64),
    Dir(String),
}

impl Filesystem {
    fn new_file(name: String, size: i64) -> Filesystem {
        Filesystem::File(name.to_owned(), size)
    }

    fn new_dir(name: String) -> Filesystem {
        Filesystem::Dir(name.to_owned())
    }
}

struct Device {
    working_directory: Vec<String>,
    filesystem: HashMap<String, Filesystem>,
}

impl Device {
    fn new() -> Self {
        Self { working_directory: Vec::new(), filesystem: HashMap::new() }
    }

    fn cwd(&self) -> String {
        self.working_directory.join("/") + "/"
    }

    fn size_of(&self, dir: String) -> i64 {
        self.filesystem
            .iter()
            .filter(|(path, _)| path.starts_with(&dir))
            .fold(0, |acc, (_, node)| match node {
                Filesystem::File(_, size) => acc + size,
                _ => acc
            })
    }

    fn parse_cd(&mut self, input: &str) -> Result<(), ()> {
        let parts: Vec<&str> = input.split("$ cd ").collect();

        if parts.len() == 2 {
            if parts[1] == ".." {
                self.working_directory.pop();
            } else if parts[1] == "/" {
                self.working_directory.push("".to_string());
            } else {
                self.working_directory.push(parts[1].to_string());
            }

            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_ls(&self, input: &str) -> Result<(), ()> {
        if input.contains("$ ls") {
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_file(&mut self, input: &str) -> Result<(), ()> {
        let parts: Vec<&str> = input.split(" ").collect();

        if parts.len() == 2 {
            if let Ok(size) = parts[0].parse::<i64>() {
                self.filesystem.insert(
                    self.cwd() + parts[1],
                    Filesystem::new_file(parts[1].to_string(), size)
                );

                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn parse_dir(&mut self, input: &str) -> Result<(), ()> {
        let parts: Vec<&str> = input.split(" ").collect();

        if parts.len() == 2 && parts[0] == "dir" {
            self.filesystem.insert(
                self.cwd() + parts[1],
                Filesystem::new_dir(parts[1].to_string())
            );

            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_command(&mut self, input: &str) -> Result<(), ()> {
        Err(())
            .or(self.parse_cd(input))
            .or(self.parse_ls(input))
            .or(self.parse_file(input))
            .or(self.parse_dir(input))
    }
}

fn first_solution(input: &Vec<String>) -> String {
    let mut device = Device::new();

    for line in input.iter() {
        device.parse_command(line);
    }

    let folders: i64 = device.filesystem.iter()
        .filter_map(|(path, node)| match node {
            Filesystem::Dir(_) => Some(path),
            _ => None
        })
        .map(|dir| device.size_of(dir.to_string()))
        .filter(|size| size <= &100000)
        .sum();

    format!("{}", folders)
}

fn second_solution(input: &Vec<String>) -> String {
    let mut device = Device::new();

    for line in input.iter() {
        device.parse_command(line);
    }

    let folders: Vec<(String, i64)> = device.filesystem.iter()
        .filter_map(|(path, node)| match node {
            Filesystem::Dir(_) => Some(path),
            _ => None
        })
        .map(|dir| (dir.to_string(), device.size_of(dir.to_string())))
        .collect();

    let capacity = 70000000;
    let required_space = 30000000;
    let used_space = device.size_of("/".to_string());

    let mut delete_candidate = None;

    for (_, size) in folders.iter() {
        if capacity - used_space + size >= required_space {
            if let Some(prev_smallest) = delete_candidate {
                if size < prev_smallest {
                    delete_candidate = Some(size);
                }
            } else {
                delete_candidate = Some(size);
            }
        }
    }

    format!("{}", delete_candidate.unwrap())
}
