use std::fs;

fn read_input() -> Vec<i32> {
    let file_path = "inputs/day01.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve1() {
    let input = read_input();
    let result = input.windows(2).filter(|w| w[0] < w[1]).count();
    println!("{result}")
}

pub fn solve2() {
    let input = read_input();
    let result = input
        .windows(3)
        .map(|w| w.into_iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();
    println!("{result}")
}
