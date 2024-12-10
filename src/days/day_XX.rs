use super::{AdventDay, Solution};

pub struct DayXX;

impl AdventDay for DayXX {
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
    fn dayXX_1() {
        let res = DayXX {}.part_1();
        assert_eq!(res, Solution::Number(1));
    }

    #[test]
    fn dayXX_2() {
        let res = DayXX {}.part_2();
        assert_eq!(res, Solution::Number(1));
    }
}
