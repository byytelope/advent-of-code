use std::{any::type_name, fs::read_to_string};

pub mod day01;
pub mod day02;

pub trait AdventDay {
    fn run(&self) -> (isize, isize) {
        let full_name = type_name::<Self>();
        let day_number = full_name
            .split("::")
            .last()
            .unwrap_or(full_name)
            .trim_start_matches("Day")
            .parse::<u8>()
            .expect("SHOULD NOT HAPPEN!");

        let main_path = format!("data/input/{}.txt", day_number);
        let eg1_path = format!("data/example/{}-1.txt", day_number);
        let eg2_path = format!("data/example/{}-2.txt", day_number);

        let main_input = read_to_string(&main_path)
            .unwrap_or_else(|_| panic!("ERROR: Failed to read input for day {}", day_number));
        let eg1_input = read_to_string(&eg1_path)
            .unwrap_or_else(|_| panic!("ERROR: Failed to read eg 1 input for day {}", day_number));
        let eg2_input = read_to_string(&eg2_path).unwrap_or(eg1_input.clone());

        let inputs = if cfg!(test) {
            (&eg1_input, &eg2_input)
        } else {
            (&main_input, &main_input)
        };

        (self.part1(inputs.0), self.part2(inputs.1))
    }
    fn part1(&self, input: &str) -> isize;
    fn part2(&self, input: &str) -> isize;
}
