#![allow(unused)]

use aoc_2023::utils::get_input;

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first_num = line
                .chars()
                .by_ref()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .next()
                .unwrap_or('0');

            let last_num = line
                .chars()
                .by_ref()
                .rev()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .next()
                .unwrap_or('0');

            format!("{}{}", first_num, last_num).parse::<i32>().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let num_strs = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3ree"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "s7ven"),
        ("eight", "e8ght"),
        ("nine", "n9ne"),
    ];

    let parsed = input
        .lines()
        .map(|line| {
            num_strs
                .iter()
                .fold(line.to_string(), |ln, (digit, rep)| ln.replace(digit, rep))
                + "\n"
        })
        .collect::<String>();

    part_one(&parsed)
}

fn main() {
    let input = get_input(1);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_1() {
        assert_eq!(
            part_one(
                "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn day01_2() {
        assert_eq!(
            part_two(
                "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
            ),
            281
        );
    }
}
