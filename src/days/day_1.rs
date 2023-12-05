use regex::Regex;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_1.txt").expect("Error reading the file")
}

pub fn first() {
    let sum: u32 = read_file()
        .split_terminator('\n')
        .map(|line| {
            let vec: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            vec.first().unwrap() * 10 + vec.last().unwrap()
        })
        .sum();
    println!("{}", sum);
}

fn get_number(line: &str) -> usize {
    let digits = [
        ["1", "one"],
        ["2", "two"],
        ["3", "three"],
        ["4", "four"],
        ["5", "five"],
        ["6", "six"],
        ["7", "seven"],
        ["8", "eight"],
        ["9", "nine"],
    ];
    let first_digit = digits
        .iter()
        .enumerate()
        .filter_map(|(i, x)| Some((i, x.iter().filter_map(|sub| line.find(sub)).min()?)))
        .min_by_key(|(_, position)| *position)
        .unwrap()
        .0
        + 1;

    let last_digit = digits
        .iter()
        .enumerate()
        .filter_map(|(i, x)| Some((i, x.iter().filter_map(|sub| line.rfind(sub)).max()?)))
        .max_by_key(|(_, position)| *position)
        .unwrap()
        .0
        + 1;

    first_digit * 10 + last_digit
}

pub fn second() {
    let sum: usize = read_file().split_terminator('\n').map(get_number).sum();
    println!("{}", sum);
}

pub fn regex() {
    let re = Regex::new(r"(?m)^.*?(\d|one|two|three|four|five|six|seven|eight|nine)(?:.*(\d|one|two|three|four|five|six|seven|eight|nine))?.*?$").unwrap();
    let result: u32 = re
        .captures_iter(&read_file())
        .map(|x| {
            let first = match x.get(1).unwrap().as_str() {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => 0,
            };
            let second = match x.get(2).map(|x| x.as_str()).unwrap_or("") {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                "" => first,
                _ => 0,
            };
            first * 10 + second
        })
        .sum();
    println!("Regex: {}", result);
    second();
}
