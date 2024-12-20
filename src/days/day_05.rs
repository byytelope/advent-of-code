use std::collections::{HashMap, HashSet, VecDeque};

use super::{AdventDay, Solution};

type Rules = HashMap<usize, HashSet<usize>>;

pub struct Day05;

impl Day05 {
    fn parse_input(&self, input: &str) -> (Rules, Vec<Vec<usize>>) {
        let (rules_str, updates_str) = input.split_once("\n\n").unwrap_or_default();
        let rules =
            rules_str
                .lines()
                .fold(HashMap::<usize, HashSet<usize>>::new(), |mut acc, line| {
                    let (first, second) = line
                        .split_once('|')
                        .map(|(first, second)| {
                            (
                                first.parse::<usize>().unwrap_or_else(|_| {
                                    panic!("ERROR: Couldn't parse `{}` into usize", first)
                                }),
                                second.parse::<usize>().unwrap_or_else(|_| {
                                    panic!("ERROR: Couldn't parse `{}` into usize", second)
                                }),
                            )
                        })
                        .unwrap();

                    acc.entry(first).or_default().insert(second);

                    acc
                });

        let updates = updates_str
            .lines()
            .map(|update_str| {
                update_str
                    .trim()
                    .split(',')
                    .map(|x| {
                        x.parse::<usize>()
                            .unwrap_or_else(|_| panic!("ERROR: Couldn't parse `{}` into usize", x))
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        (rules, updates)
    }

    fn check_valid(&self, rules: &Rules, update: &[usize]) -> bool {
        update
            .windows(2)
            .map(|window| {
                let first = window[0];
                let second = window[1];

                if let Some(rule) = rules.get(&first) {
                    rule.contains(&second)
                } else if let Some(rule) = rules.get(&second) {
                    !rule.contains(&first)
                } else {
                    true
                }
            })
            .all(|x| x)
    }

    fn top_sort(&self, rules: &Rules, update: &[usize]) -> Option<Vec<usize>> {
        let rule_nums = rules
            .keys()
            .chain(rules.values().flatten())
            .copied()
            .collect::<HashSet<usize>>();

        let fixed_nums = update
            .iter()
            .filter(|&n| !rule_nums.contains(n))
            .enumerate()
            .map(|(i, &n)| (i, n))
            .collect::<HashMap<usize, usize>>();

        let mut in_degree = update
            .iter()
            .filter(|&&n| rule_nums.contains(&n))
            .map(|&n| (n, 0))
            .collect::<HashMap<usize, usize>>();

        for (&from, to) in rules {
            if in_degree.contains_key(&from) {
                for &to in to {
                    if in_degree.contains_key(&to) {
                        *in_degree.get_mut(&to).unwrap() += 1;
                    }
                }
            }
        }

        let mut sorted = Vec::new();
        let mut queue = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&n, _)| n)
            .collect::<VecDeque<usize>>();

        while let Some(n) = queue.pop_front() {
            sorted.push(n);
            if let Some(neighbors) = rules.get(&n) {
                for &m in neighbors {
                    if let Some(degree) = in_degree.get_mut(&m) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(m);
                        }
                    }
                }
            }
        }

        if sorted.len() != in_degree.len() {
            return None;
        }

        let mut result = vec![0; update.len()];
        let mut sorted_idx = 0;

        for (i, _) in update.iter().enumerate() {
            result[i] = match fixed_nums.get(&i) {
                Some(&n) => n,
                None => {
                    let n = sorted.get(sorted_idx).copied()?;
                    sorted_idx += 1;

                    n
                }
            };
        }

        Some(result)
    }
}

impl AdventDay for Day05 {
    fn part_1(&self) -> Solution {
        let input = self.get_input().0;
        let (rules, updates) = self.parse_input(&input);

        let res = updates
            .iter()
            .filter_map(|update| match self.check_valid(&rules, update) {
                true => Some(update[update.len() / 2]),
                false => None,
            })
            .sum::<usize>();

        Solution::Number(res)
    }

    fn part_2(&self) -> Solution {
        let input = self.get_input().1;
        let (rules, updates) = self.parse_input(&input);
        let res = updates
            .iter()
            .filter(|x| !self.check_valid(&rules, x))
            .map(|update| {
                if let Some(updated) = self.top_sort(&rules, update) {
                    updated[updated.len() / 2]
                } else {
                    0
                }
            })
            .sum::<usize>();

        Solution::Number(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05p1() {
        let res = Day05.part_1();
        assert_eq!(res, Solution::Number(143));
    }

    #[test]
    fn day_05p2() {
        let res = Day05.part_2();
        assert_eq!(res, Solution::Number(123));
    }
}
