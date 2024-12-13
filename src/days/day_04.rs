use super::{AdventDay, Solution};

static DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];

pub struct Day04;

impl AdventDay for Day04 {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let n_row = grid.len();
        let n_col = grid[0].len();
        let mut res = 0;

        for c_row in 0..n_row {
            for c_col in 0..n_col {
                'dir_loop: for (dir_x, dir_y) in DIRECTIONS.iter() {
                    for (i, char) in "XMAS".char_indices() {
                        let r = (c_row as isize + (i as isize * dir_x)) as usize;
                        let c = (c_col as isize + (i as isize * dir_y)) as usize;

                        if r >= n_row || c >= n_col || grid[r][c] != char {
                            continue 'dir_loop;
                        }
                    }

                    res += 1;
                }
            }
        }

        Solution::Number(res)
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let n_row = grid.len();
        let n_col = grid[0].len();
        let mut res = 0;

        for c_row in 1..n_row - 1 {
            for c_col in 1..n_col - 1 {
                if grid[c_row][c_col] != 'A' {
                    continue;
                }

                let diag_1 = (grid[c_row - 1][c_col - 1], grid[c_row + 1][c_col + 1]);

                let diag_2 = (grid[c_row - 1][c_col + 1], grid[c_row + 1][c_col - 1]);

                if matches!(diag_1, ('M', 'S') | ('S', 'M'))
                    && matches!(diag_2, ('M', 'S') | ('S', 'M'))
                {
                    res += 1;
                }
            }
        }

        Solution::Number(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04p1() {
        let res = Day04.part_1();
        assert_eq!(res, Solution::Number(18));
    }

    #[test]
    fn day_04p2() {
        let res = Day04.part_2();
        assert_eq!(res, Solution::Number(9));
    }
}
