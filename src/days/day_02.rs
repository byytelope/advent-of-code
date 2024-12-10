use super::{AdventDay, Solution};

pub struct Day02;

impl AdventDay for Day02 {
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
    fn day02_1() {
        let res = Day02 {}.part_1();
        assert_eq!(res, Solution::Number(2));
    }

    #[test]
    fn day02_2() {
        let res = Day02 {}.part_2();
        assert_eq!(res, Solution::Number(4));
    }
}
