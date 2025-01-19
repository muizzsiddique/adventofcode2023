use std::str::FromStr;
use std::fs;

const FILEPATH: &str = "aoc03.txt";

fn main() {

    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let mut solution = 0;

    // Create a freely navigable 2D vector of the `input`.
    let puzzle_bytes: Vec<Vec<_>> = puzzle.split_terminator('\n').map( |line| line.chars().collect() ).collect();

    let mut y = 0;
    while y < puzzle_bytes.len() {
        let mut x = 0;
        while x < puzzle_bytes[y].len() {

            let character = puzzle_bytes[y][x];
            if is_digit(character) {
                let len: usize = {
                    puzzle_bytes[y].iter().skip(x).position(|&c| !is_digit(c)).unwrap_or(puzzle_bytes[y].len()-x)
                };

                let string_of_digits: String = puzzle_bytes[y].iter().skip(x).take(len).collect();

                let is_part_top = if y >= 1 { // Unsigned shenangians
                    let x = std::cmp::max(x, 1) - 1; // Unsigned shenangians
                    puzzle_bytes[y-1].iter().skip(x).take(len+2).find(|&&c| is_symbol(c)).is_some()
                } else {
                    false
                };
                let is_part_bottom = if y < puzzle_bytes.len() - 1 { // Unsigned shenangians
                    let x = std::cmp::max(x, 1) - 1; // Unsigned shenangians
                    puzzle_bytes[y+1].iter().skip(x).take(len+2).find(|&&c| is_symbol(c)).is_some()
                } else {
                    false
                };
                let is_part_middle = {
                    let x = std::cmp::max(x, 1) - 1; // Unsigned shenangians
                    puzzle_bytes[y].iter().skip(x).take(len+2).find(|&&c| is_symbol(c)).is_some()
                };

                if is_part_top || is_part_middle || is_part_bottom {
                    solution += usize::from_str(&string_of_digits).unwrap();
                }

                x += len;
                continue;
            }
            x += 1;
        }
        y += 1;
    }

    println!("{solution}");
}

fn is_digit(c: char) -> bool {
    "0123456789".contains(c)
}

fn is_symbol(c: char) -> bool {
    !"0123456789.".contains(c)
}
