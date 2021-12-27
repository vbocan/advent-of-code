use std::fs::File;
use std::io::{prelude::*, BufReader};

// Solution for Advent of Code 2015-01 (https://adventofcode.com/2015/day/2)
fn main() {
    let file = File::open("input.txt").expect("Input file not found!");
    let reader = BufReader::new(file);
    let mut total_area = 0;
    let mut ribbon_length = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let parts = l
            .split_terminator('x')
            .map(|s| s.parse::<i32>())
            .collect::<Vec<_>>();
        let length = *(parts[0].as_ref()).unwrap();
        let width = *(parts[1].as_ref()).unwrap();
        let height = *(parts[2].as_ref()).unwrap();

        println!("Length: {}, Width: {}, Height: {}", length, width, height);

        let surface_area = 2 * length * width + 2 * width * height + 2 * height * length;
        let ss = smallest_side((length, width, height));
        println!("Smallest side: {:?}", ss);

        let surface_area = surface_area + ss.0 * ss.1;
        total_area += surface_area;

        ribbon_length += (ss.0 * 2 + ss.1 * 2) + (length * width * height);
    }
    println!("Total surface: {} sq ft", total_area);
    println!("Ribbon length: {} ft", ribbon_length);
}

fn smallest_side(sides: (i32, i32, i32)) -> (i32, i32) {
    let length = sides.0;
    let width = sides.1;
    let height = sides.2;
    // Get the smallest side
    if length >= width && length >= height {
        // Smallest side is width - height
        (width, height)
    } else if width >= length && width >= height {
        // Smallest side is length - height
        (length, height)
    } else {
        // Smallest side is length - width
        (length, width)
    }
}
