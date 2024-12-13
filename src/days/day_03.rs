use super::{AdventDay, Solution};

pub struct Day03;

impl Day03 {
    fn get_res(&self, input: &str) -> usize {
        let mul_matches = input.match_indices("mul(");
        let paran_matches = input.match_indices(')');

        mul_matches
            .filter_map(|(mul_i, _)| {
                paran_matches
                    .clone()
                    .find(|(paran_i, _)| mul_i <= *paran_i && (paran_i - mul_i >= 7))
                    .and_then(|(paran_i, _)| {
                        input[mul_i + 4..paran_i]
                            .split_once(',')
                            .and_then(|(l_str, r_str)| {
                                l_str
                                    .parse::<usize>()
                                    .ok()
                                    .zip(r_str.parse::<usize>().ok())
                                    .map(|(l_num, r_num)| l_num * r_num)
                            })
                    })
            })
            .sum::<usize>()
    }
}

impl AdventDay for Day03 {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;
        let res = self.get_res(&input);

        Solution::Number(res)
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;
        let parsed_input = input
            .split("do()")
            .map(|d| d.split("don't()").next().unwrap_or_default())
            .collect::<String>();
        let res = self.get_res(&parsed_input);

        Solution::Number(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03p1() {
        let res = Day03.part_1();
        assert_eq!(res, Solution::Number(161));
    }

    #[test]
    fn day_03p2() {
        let res = Day03.part_2();
        assert_eq!(res, Solution::Number(48));
    }
}
