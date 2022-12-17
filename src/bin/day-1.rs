mod util;
use util::read_file::read_file;

fn main() {
    let mut elves = read_file(1, false)
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    elves.sort();
    elves.reverse();
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
}

fn part1(elves: &[u32]) -> u32 {
    elves[0]
}

fn part2(elves: &[u32]) -> u32 {
    elves[0] + elves[1] + elves[2]
}
