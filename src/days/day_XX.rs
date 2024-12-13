use super::{AdventDay, Solution};

pub struct DayXX;

impl AdventDay for DayXX {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;

        Solution::Number(0)
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;

        Solution::Number(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_XXp1() {
        let res = DayXX.part_1();
        assert_eq!(res, Solution::Number(1));
    }

    #[test]
    fn day_XXp2() {
        let res = DayXX.part_2();
        assert_eq!(res, Solution::Number(1));
    }
}
