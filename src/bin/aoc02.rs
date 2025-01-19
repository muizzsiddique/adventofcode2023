use std::str::FromStr;
use std::fs;
use std::collections::HashMap;

// ASSUMPTION: I'm trusting the formatting of the file.

const FILEPATH: &str = "aoc02.txt";

fn main() {
    let maximums = HashMap::<&str, usize>::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    let solution = puzzle
        .split_terminator('\n')
        .map(|line| {
            let splitted: Vec<_> = line.split(':').collect();
            if let [id_str, sets_str] = splitted[..] {
                let id = usize::from_str(&id_str[5..]).unwrap();

                for set_str in sets_str.split(';') {
                    for cube_str in set_str.split(",") {
                        for (colour, max) in maximums.iter() {
                            if cube_str.contains(colour) {
                                let count = usize::from_str(&cube_str[1..cube_str.len()-1-colour.len()]).unwrap();
                                if &count > max {
                                    return 0;
                                }
                            }
                        }
                    }
                }
                return id;
            }
            unreachable!();
        })
        .reduce(|x, y| x + y)
        .unwrap();

    println!("{:?}", solution);
}
