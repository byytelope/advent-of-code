use super::{AdventDay, Solution};

pub struct Day02;

impl Day02 {
    fn get_rows(&self, input: &str) -> Vec<Vec<isize>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| {
                        x.parse::<isize>()
                            .unwrap_or_else(|_| panic!("ERROR: Couldn't parse `{}` as isize", x))
                    })
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>()
    }

    fn check_row(&self, row: &[isize]) -> bool {
        let sig = (row[0] - row[1]).signum();
        row.windows(2)
            .map(|w| w[0] - w[1])
            .all(|x| (1..=3).contains(&x.abs()) && x.signum() == sig)
    }
}

impl AdventDay for Day02 {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;
        let rows = self.get_rows(&input);
        let res = rows
            .iter()
            .map(|row| self.check_row(row))
            .filter(|x| *x)
            .count();

        Solution::Number(res)
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;
        let rows = self.get_rows(&input);

        let res = rows
            .iter()
            .map(|row| {
                (0..row.len())
                    .map(|i| {
                        let (before, after) = row.split_at(i);
                        let _row = before
                            .iter()
                            .chain(&after[1..])
                            .cloned()
                            .collect::<Vec<isize>>();
                        self.check_row(&_row)
                    })
                    .any(|x| x)
            })
            .filter(|x| *x)
            .count();

        Solution::Number(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02p1() {
        let res = Day02 {}.part_1();
        assert_eq!(res, Solution::Number(2));
    }

    #[test]
    fn day_02p2() {
        let res = Day02 {}.part_2();
        assert_eq!(res, Solution::Number(4));
    }
}
