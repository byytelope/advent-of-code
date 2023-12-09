#![allow(unused)]

use aoc_2023::utils::get_input;

fn part_one(input: &str) -> u32 {
    0
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = get_input(0);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day00_1() {
        assert_eq!(part_one(""), 0);
    }

    #[test]
    fn day00_2() {
        assert_eq!(part_two(""), 0);
    }
}
