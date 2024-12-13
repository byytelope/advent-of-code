use std::{any::type_name, fs::read_to_string};

pub mod day_01;
pub mod day_02;
pub mod day_03;

#[derive(PartialEq, Debug)]
pub enum Solution {
    Number(usize),
    Str(String),
}

pub trait AdventDay {
    fn get_input(&self) -> (String, String) {
        let full_name = type_name::<Self>();
        let day_number = full_name
            .split("::")
            .last()
            .unwrap_or(full_name)
            .trim_start_matches("Day")
            .parse::<u8>()
            .unwrap_or_else(|_| {
                panic!(
                    "ERROR: `{}` should follow convention `DayXX`: XX -> 01, 02 etc.",
                    full_name
                )
            });
        let main_path = format!("data/input/{}.txt", day_number);
        let eg1_path = format!("data/example/{}-1.txt", day_number);
        let eg2_path = format!("data/example/{}-2.txt", day_number);

        let main_input = read_to_string(&main_path)
            .unwrap_or_else(|_| panic!("Failed to read input for day `{}`", day_number));
        let eg1_input = read_to_string(&eg1_path).unwrap_or_else(|_| {
            panic!("ERROR: Failed to read eg 1 input for day `{}`", day_number)
        });
        let eg2_input = read_to_string(&eg2_path).unwrap_or(eg1_input.clone());

        if cfg!(test) {
            (eg1_input.clone(), eg2_input.clone())
        } else {
            (main_input.clone(), main_input.clone())
        }
    }

    fn part_1(&self) -> Solution;
    fn part_2(&self) -> Solution;
}
