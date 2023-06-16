use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value) = s.split_once(' ').unwrap();
        let value = value.parse().unwrap();
        match command {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(()),
        }
    }
}

fn read_input() -> Vec<Command> {
    let file_path = "inputs/day02.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve1() {
    let commands = read_input();
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(x) => horizontal += x,
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x,
        }
    }

    let result = horizontal * depth;
    println!("{result}")
}

pub fn solve2() {
    let commands = read_input();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }

    let result = horizontal * depth;
    println!("{result}")
}
