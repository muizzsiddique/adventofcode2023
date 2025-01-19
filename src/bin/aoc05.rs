use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

const FILEPATH: &str = "aoc05.txt";

fn main() {
    let puzzle = fs::read_to_string(FILEPATH).unwrap();

    //                           map,                    xfrom
    let mut steps = vec![Vec::<(std::ops::Range<isize>, isize)>::new(); 7]; // 7 Steps

    let mut seeds: Vec<isize> = puzzle
        .split_terminator('\n')
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| isize::from_str(s).unwrap())
        .collect();

    let mut iter = puzzle.split_terminator('\n').skip(3);
    let mut index = 0;
    while let Some(line) = iter.next() {
        if line == "" {
            iter.next();
            index += 1;
            continue;
        }
        let [dest, source, len] = line.split_whitespace().collect::<Vec<_>>()[..] else {
            panic!();
        };
        let (Ok(dest), Ok(source), Ok(len)) = (
            isize::from_str(dest),
            isize::from_str(source),
            isize::from_str(len),
        ) else {
            panic!();
        };
        steps[index].push((source..(source + len), dest - source));
    }

    for seed in &mut seeds {
        for step in &steps {
            for (map, xform) in step {
                if map.contains(&seed) {
                    *seed = seed.clone() + xform;
                    break;
                }
            }
        }
    }

    println!("{:?}", seeds.iter().min().unwrap());
}
