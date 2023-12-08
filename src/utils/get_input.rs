use std::fs::read_to_string;

pub fn get_input(day: u8) -> String {
    read_to_string(format!("src/inputs/day{:02}.txt", day)).unwrap()
}
