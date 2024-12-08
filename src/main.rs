use days::{day01::Day01, day02::Day02, AdventDay};

pub mod days;

fn main() {
    let advent_days: Vec<Box<dyn AdventDay>> = vec![Box::new(Day01 {}), Box::new(Day02 {})];

    advent_days.iter().for_each(|day| {
        day.run();
    });
}
