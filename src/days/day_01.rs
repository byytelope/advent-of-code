use super::{AdventDay, Solution};

pub struct Day01;

impl AdventDay for Day01 {
    fn part_1(&self) -> Solution {
        Solution::Number(0)
    }

    fn part_2(&self) -> Solution {
        Solution::Number(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_1() {
        let res = Day01 {}.part_1();
        assert_eq!(res, Solution::Number(11));
    }

    #[test]
    fn day01_2() {
        let res = Day01 {}.part_2();
        assert_eq!(res, Solution::Number(31));
    }
}
