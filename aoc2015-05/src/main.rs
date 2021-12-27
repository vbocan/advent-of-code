use std::fs::File;
use std::io::{prelude::*, BufReader};

// Solution for Advent of Code 2015-05 (https://adventofcode.com/2015/day/5)
fn main() {
    let file = File::open("input.txt").expect("Input file not found!");
    let reader = BufReader::new(file);
    let mut nice_strings_count = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let chk1 = check_vowels(&l);
        let chk2 = check_doubled_letters(&l);
        let chk3 = check_forbidden_strings(&l);
        if chk1 && chk2 && chk3 {
            nice_strings_count += 1;
        }
    }
    println!("Number of nice strings: {}", nice_strings_count);
}

// #region Rule Set #1
fn check_vowels(input: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    // Check for at least three vowels
    let count: i32 = input
        .chars()
        .map(|x| if vowels.contains(&x) { 1 } else { 0 })
        .sum();
    count >= 3
}

fn check_doubled_letters(input: &str) -> bool {
    let doubled_letters = vec![
        "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo",
        "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz",
    ];
    let count: i32 = doubled_letters
        .iter()
        .map(|&x| if input.contains(x) { 1 } else { 0 })
        .sum();
    count >= 1
}

fn check_forbidden_strings(input: &str) -> bool {
    let forbidden_strings = vec!["ab", "cd", "pq", "xy"];
    let count: i32 = forbidden_strings
        .iter()
        .map(|&x| if input.contains(x) { 1 } else { 0 })
        .sum();
    count == 0
}
// #endregion