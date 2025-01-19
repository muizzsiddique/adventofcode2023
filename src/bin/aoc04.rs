use std::fs;
use std::str::FromStr;

const FILEPATH: &str = "aoc04.txt";

fn main() {
    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let mut solution = 0;

    puzzle.split_terminator('\n').for_each(|line| {
        if let [winning_nums, your_nums] = line
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .collect::<Vec<_>>()[..]
        {
            let winning_nums = winning_nums
                .trim()
                .split_whitespace()
                .map(|s| usize::from_str(s).unwrap())
                .collect::<Vec<_>>();
            let your_nums = your_nums
                .trim()
                .split_whitespace()
                .map(|s| usize::from_str(s).unwrap())
                .collect::<Vec<_>>();

            let mut score = 1;
            for your_num in your_nums {
                if winning_nums.contains(&your_num) {
                    score *= 2;
                }
            }
            solution += score / 2;
        }
    });

    println!("{:?}", solution);
}
