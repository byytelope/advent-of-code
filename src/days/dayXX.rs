pub use super::AdventDay;

pub struct DayXX;

impl AdventDay for DayXX {
    fn new() -> Self {
        Self
    }

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
    fn dayXX_1() {
        let (part1, _) = DayXX::new().run();
        assert_eq!(part1, 1);
    }

    #[test]
    fn dayXX_2() {
        let (_, part2) = DayXX::new().run();
        assert_eq!(part2, 1);
    }
}
