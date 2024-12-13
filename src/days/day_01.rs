use std::iter::zip;

use super::{AdventDay, Solution};

pub struct Day01;

impl Day01 {
    fn get_cols(&self, input: &str) -> (Vec<usize>, Vec<usize>) {
        let mut l_col = Vec::<usize>::new();
        let mut r_col = Vec::<usize>::new();

        input.trim().lines().for_each(|line| {
            let (l_num_str, r_num_str) = match line.find(' ') {
                Some(space_idx) => (&line[0..space_idx], &line[space_idx + 3..]),
                None => panic!("Incorrect input format"),
            };

            l_col.push(
                l_num_str
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Couldn't parse `{}` into an usize", l_num_str)),
            );
            r_col.push(
                r_num_str.parse::<usize>().unwrap_or_else(|_| {
                    panic!("ERROR: Couldn't parse `{}` into an usize", r_num_str)
                }),
            );
        });

        (l_col, r_col)
    }
}

impl AdventDay for Day01 {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;
        let (mut l_col, mut r_col) = self.get_cols(&input);

        l_col.sort();
        r_col.sort();

        Solution::Number(
            zip(&l_col, &r_col)
                .map(|(l, r)| l.abs_diff(*r))
                .sum::<usize>(),
        )
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;
        let (l_col, r_col) = self.get_cols(&input);

        Solution::Number(
            l_col
                .iter()
                .map(|l| l * r_col.iter().filter(|r| *r == l).count())
                .sum::<usize>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01p1() {
        let res = Day01.part_1();
        assert_eq!(res, Solution::Number(11));
    }

    #[test]
    fn day_01p2() {
        let res = Day01.part_2();
        assert_eq!(res, Solution::Number(31));
    }
}
