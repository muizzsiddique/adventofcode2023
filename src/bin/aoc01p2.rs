use std::fs;
use std::str::FromStr;

const FILEPATH: &str = "aoc01.txt";

const NUM_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let solution = puzzle
        .split_terminator('\n')
        .inspect(|line| println!("         Source: {:?}", line))
        .map(|line| convert_to_digits(line))
        .inspect(|line| println!("Words-to-digits: {:?}", line))
        .map(|line| get_calibration_value(&line))
        .inspect(|num| println!(" Concat'd number: {:?}", num))
        .reduce(|x, y| x + y)
        .unwrap_or(0);

    println!("{:#?}", solution);
}

fn convert_to_digits(line: &str) -> String {
    let mut words = Vec::new();

    for word in NUM_WORDS {
        words.extend_from_slice(line.match_indices(word).collect::<Vec<_>>().as_slice());
    }

    if words.len() == 0 {
        return String::from(line);
    }

    let mut left_index = words[0].0;
    let mut left_word = words[0].1;

    for (next_index, next_word) in words.iter() {
        if *next_index < left_index {
            left_word = next_word;
            left_index = *next_index;
        }
    }

    let mut right_index = words[0].0;
    let mut right_word = words[0].1;

    for (next_index, next_word) in words.iter() {
        if *next_index > right_index {
            right_word = next_word;
            right_index = *next_index;
        }
    }

    // println!("left = {:?}, right = {:?}", left_word, right_word);

    line.replacen(
        left_word,
        &Vec::from(NUM_WORDS)
            .iter()
            .position(|&x| x == left_word)
            .unwrap()
            .to_string(),
        1,
    )
    .replace(
        right_word,
        &Vec::from(NUM_WORDS)
            .iter()
            .position(|&x| x == right_word)
            .unwrap()
            .to_string(),
    )
}

fn get_calibration_value(line: &str) -> usize {
    let mut digits_in_string = line.matches(|x| ('0'..='9').contains(&x));
    let left_char = digits_in_string.next().unwrap();
    let right_char = digits_in_string.next_back().unwrap_or(left_char);

    let number_string = format!("{}{}", left_char, right_char);
    return usize::from_str(&number_string).unwrap();
}
