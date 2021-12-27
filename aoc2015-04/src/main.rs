use md5;

// Solution for Advent of Code 2015-04 (https://adventofcode.com/2015/day/4)

const PUZZLE_INPUT: &str = "yzbqklnj";
const PATTERN: &str = "00000";

fn main() {
    for i in 10000..100000000 {
        let s = format!("{}{:06}", PUZZLE_INPUT, i);
        let digest = format!("{:x}", md5::compute(&s));
        if digest.starts_with(PATTERN) {
            println!("Solution: {}", i);
            break;
        }
    }
}
