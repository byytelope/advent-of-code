use days::{day_01::Day01, day_02::Day02, AdventDay};

pub mod days;

fn main() {
    let advent_days: Vec<Box<dyn AdventDay>> = vec![Box::new(Day01 {}), Box::new(Day02 {})];

    advent_days.iter().enumerate().for_each(|(i, day)| {
        println!("Day {}_1: {:#?}", i + 1, day.part_1());
        println!("Day {}_1: {:#?}", i + 1, day.part_2());
    });
}
