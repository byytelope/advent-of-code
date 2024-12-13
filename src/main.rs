use days::{day_01::Day01, day_02::Day02, day_03::Day03, AdventDay};

pub mod days;

fn main() {
    let advent_days: Vec<Box<dyn AdventDay>> =
        vec![Box::new(Day01), Box::new(Day02), Box::new(Day03)];

    advent_days.iter().enumerate().for_each(|(i, day)| {
        println!("Day {} P1: {:?}", i + 1, day.part_1());
        println!("Day {} P2: {:?}\n", i + 1, day.part_2());
    });
}
