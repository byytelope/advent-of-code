use days::{day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04, day_05::Day05, AdventDay};

pub mod days;

fn main() {
    let panic_msg = "Usage: [1-25]p[1|2]";
    let args = std::env::args().skip(1);

    let advent_days: Vec<Box<dyn AdventDay>> = vec![
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
    ];

    match args.len() {
        0 => {
            advent_days.iter().enumerate().for_each(|(i, day)| {
                println!("Day {}", i + 1);
                println!("P1: {:?}", day.part_1());
                println!("P2: {:?}\n", day.part_2());
            });
        }
        _ => {
            args.for_each(|arg| match arg.split_once('p') {
                Some((day_str, part_str)) => {
                    let day_num = day_str
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("{panic_msg}"));
                    let part_num = part_str
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("{panic_msg}"));

                    if let Some(day) = advent_days.get(day_num - 1) {
                        println!("Day {}", day_num);
                        match part_num {
                            1 => println!("P1: {:?}", day.part_1()),
                            2 => println!("P2: {:?}", day.part_2()),
                            _ => panic!("{panic_msg}"),
                        }
                    } else {
                        panic!("{panic_msg}");
                    }
                }
                None => {
                    let day_num = arg
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("{panic_msg}"));

                    if let Some(day) = advent_days.get(day_num - 1) {
                        println!("Day {}", day_num);
                        println!("P1: {:?}", day.part_1());
                        println!("P2: {:?}", day.part_2());
                    } else {
                        panic!("{panic_msg}");
                    }
                }
            });
        }
    }
}
