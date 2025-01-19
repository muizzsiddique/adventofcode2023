use std::fs;

const FILEPATH: &str = "aoc01.txt";

const NUMBER_STRS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let mut solution = 0;

    for line in puzzle.lines() {
        println!("{:?}", &line);
        let line = replace_left_word(&line);
        let line = replace_right_word(&line);
        println!("{:?}", &line);
        {
            let chars = line.chars();
            let left = chars.clone().find(|&c| c >= '0' && c <= '9').unwrap();
            let right = chars.rev().find(|&c| c >= '0' && c <= '9').unwrap();
            println!("{}{}", &left, &right);
            solution += left.to_digit(10).unwrap() * 10 + right.to_digit(10).unwrap();
        }
    }

    println!("{:?}", solution);
}

fn replace_left_word(phrase: &str) -> String {
    let left = NUMBER_STRS
        .iter()
        .enumerate()
        .map(|(num, num_word)| (phrase.find(num_word), num))
        .filter(|(i, v)| i.is_some())
        .reduce(|a, b| if a.0 < b.0 { a } else { b });
    if let Some((_, num)) = left {
        phrase.replacen(NUMBER_STRS[num], &num.to_string(), 1)
    } else {
        phrase.to_owned()
    }
}

fn replace_right_word(phrase: &str) -> String {
    let right = NUMBER_STRS
        .iter()
        .enumerate()
        .map(|(num, num_word)| (phrase.rfind(num_word), num))
        .filter(|(i, v)| i.is_some())
        .reduce(|a, b| if a.0 > b.0 { a } else { b });
    if let Some((_, num)) = right {
        phrase.replace(NUMBER_STRS[num], &num.to_string())
    } else {
        phrase.to_owned()
    }
}

fn part1() {
    let puzzle = "1abc2\n\
                  pqr3stu8vwx\n\
                  a1b2c3d4e5f\n\
                  treb7uchet";

    let mut solution = 0;

    for line in puzzle.lines() {
        {
            let chars = line.chars();
            let left = chars.clone().find(|&c| c >= '0' && c <= '9').unwrap();
            let right = chars.rev().find(|&c| c >= '0' && c <= '9').unwrap();
            solution += left.to_digit(10).unwrap() * 10 + right.to_digit(10).unwrap();
        }
    }

    println!("{:?}", solution);
}
