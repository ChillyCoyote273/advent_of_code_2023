use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_5.txt").expect("Error reading the file")
}

pub fn first() {}

pub fn second() {}
