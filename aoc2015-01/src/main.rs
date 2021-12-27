use std::fs;

// Solution for Advent of Code 2015-01 (https://adventofcode.com/2015/day/1)
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Input file not found!");

    println!("{}", contents);

    let mut floor = 0;
    let mut index_for_basement = 0;

    for (i, c) in contents.chars().enumerate() {
        let floor_offset = match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        floor += floor_offset;
        if index_for_basement == 0 && floor == -1 {
            index_for_basement = i + 1;
        }
    }

    println!("Santa goes to floor: {}", floor);
    println!(
        "Index of first character that sends Santa to basement: {}",
        index_for_basement
    );
}
