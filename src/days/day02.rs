pub use super::AdventDay;

pub struct Day02;

impl AdventDay for Day02 {
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
    fn day02_1() {
        let (part1, _) = Day02 {}.run();
        assert_eq!(part1, 2);
    }

    #[test]
    fn day02_2() {
        let (_, part2) = Day02 {}.run();
        assert_eq!(part2, 4);
    }
}
