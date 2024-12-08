pub use super::AdventDay;

pub struct Day01;

impl AdventDay for Day01 {
    fn part1(&self, input: &str) -> isize {
        0
    }

    fn part2(&self, input: &str) -> isize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_1() {
        let (part1, _) = Day01 {}.run();
        assert_eq!(part1, 11);
    }

    #[test]
    fn day01_2() {
        let (_, part2) = Day01 {}.run();
        assert_eq!(part2, 31);
    }
}
