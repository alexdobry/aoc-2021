use std::cmp::Ordering;
use std::fs;

fn read_input() -> Vec<u16> {
    let file_path = "inputs/day03.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<u16> {
    input
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

fn most_common_bit(idx: usize, numbers: &[u16]) -> Ordering {
    let len = numbers.len();
    let one_count: usize = numbers.iter().map(|n| bit_at(idx, *n) as usize).sum();
    (one_count * 2).cmp(&len)
}

pub fn bit_at(idx: usize, number: u16) -> u16 {
    (number >> (11 - idx)) & 1
}

pub fn solve1() {
    let input = read_input();
    let mut gamma: u32 = 0;
    for d in 0..12 {
        gamma <<= 1;
        if most_common_bit(d, &input).is_gt() {
            gamma |= 1;
        }
    }
    let epsilon = gamma ^ 0b111111111111;
    let result = gamma * epsilon;
    println!("{result}")
}

pub fn solve2() {
    let input = read_input();
    let mut oxygen = input.clone();
    let mut co2 = input;
    for i in 0..12 {
        match most_common_bit(i, &oxygen) {
            Ordering::Less => {
                oxygen.retain(|n| bit_at(i, *n) == 0);
            }
            Ordering::Equal | Ordering::Greater => {
                oxygen.retain(|n| bit_at(i, *n) == 1);
            }
        }
        let co2_tmp = co2.clone();
        match most_common_bit(i, &co2) {
            Ordering::Less => {
                co2.retain(|n| bit_at(i, *n) == 1);
            }
            Ordering::Greater | Ordering::Equal => {
                co2.retain(|n| bit_at(i, *n) == 0);
            }
        }
        if co2.is_empty() {
            co2 = co2_tmp;
        }
    }

    let result: u32 = oxygen[0] as u32 * co2[0] as u32;
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::day03::{bit_at, most_common_bit};

    #[test]
    fn it_works() {
        assert_eq!(1, bit_at(0, 0b100000000000));
        assert_eq!(0, bit_at(1, 0b100000000000));
        assert_eq!(0, bit_at(11, 0b100000000000));

        assert_eq!(
            Ordering::Greater,
            most_common_bit(0, &[0b100000000000, 0b100000000000, 0b000000000000,])
        );
        assert_eq!(
            Ordering::Less,
            most_common_bit(1, &[0b100000000000, 0b000000000000, 0b100000000000,])
        );
        assert_eq!(
            Ordering::Less,
            most_common_bit(2, &[0b101000000000, 0b100000000000, 0b100000000000,])
        );
    }
}
