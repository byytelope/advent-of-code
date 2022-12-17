use core::panic;
use std::fs;

pub fn read_file(day: u8, sample: bool) -> String {
    let filename = if sample { "sample" } else { "day" };
    let path = format!("./src/inputs/{}-{}.txt", filename, day);
    let bruh = fs::read_to_string(path);
    match bruh {
        Ok(hehe) => hehe,
        Err(error) => panic!("Failed to open file: {}", error),
    }
}
