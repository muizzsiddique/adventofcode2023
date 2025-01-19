use std::str::FromStr;
use std::fs;

const FILEPATH: &str = "aoc01.txt";

fn main() {
    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let solution = puzzle.split_terminator('\n').map(|line| get_calibration_value(line)).reduce(|x, y| x + y).unwrap_or(0);

    println!("{}", solution);

}

fn get_calibration_value(line: &str) -> usize {
    let mut digits_in_string = line.matches(|x| ('0'..='9').contains(&x));
    let left_char = digits_in_string.next().unwrap();
    let right_char = digits_in_string.next_back().unwrap_or(left_char);

    let number_string = format!("{}{}", left_char, right_char);
    return usize::from_str(&number_string).unwrap();
}
