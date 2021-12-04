use std::fs;

struct Submarine {
    depth: i32,
    position: i32,
    aim: i32,
    new_version: bool,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            depth: 0,
            position: 0,
            aim: 0,
            new_version: false,
        }
    }

    fn update(&mut self, command: &Command) {
        if command.direction == "forward" {
            self.position += command.value;

            if self.new_version {
                self.depth += self.aim * command.value;
            }
        } else if command.direction == "up" {
            if self.new_version {
                self.aim -= command.value;
            } else {
                self.depth -= command.value;
            }
        } else if command.direction == "down" {
            if self.new_version {
                self.aim += command.value;
            } else {
                self.depth += command.value;
            }
        }
    }

    fn calculate(&self) -> i32 {
        self.depth * self.position
    }
}

struct Command {
    direction: String,
    value: i32,
}

fn main() {
    let input = fs::read_to_string("input/day02").expect("Error while reading");

    let commands: Vec<Command> = input
        .lines()
        .map(|command| {
            let tokens: Vec<&str> = command.split(' ').collect();
            Command {
                direction: tokens[0].to_string(),
                value: tokens[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let (mut s1, mut s2) = (Submarine::new(), Submarine::new());
    s2.new_version = true;

    for command in commands {
        s1.update(&command);
        s2.update(&command);
    }

    println!("P1: {}\nP2: {}", s1.calculate(), s2.calculate());
}
