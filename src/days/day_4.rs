use itertools::Itertools;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_4.txt").expect("Error reading the file")
}

pub fn first() {
    let score: u32 = read_file()
        .split_terminator('\n')
        .map(|line| {
            let winning_numbers: Vec<u32> = line
                .chars()
                .skip(10)
                .take(29)
                .collect::<String>()
                .split_ascii_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();
            let card_numbers: Vec<u32> = line
                .chars()
                .skip(42)
                .collect::<String>()
                .split_ascii_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();
            let mut matches = 0;
            for card_number in card_numbers {
                if winning_numbers.contains(&card_number) {
                    matches += 1;
                }
            }
            if matches == 0 {
                0
            } else {
                2u32.pow(matches - 1)
            }
        })
        .sum();

    println!("{}", score);
}

pub fn second() {
    let card_matches: Vec<u32> = read_file()
        .split_terminator('\n')
        .map(|line| {
            let winning_numbers: Vec<u32> = line
                .chars()
                .skip(10)
                .take(29)
                .collect::<String>()
                .split_ascii_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();
            let card_numbers: Vec<u32> = line
                .chars()
                .skip(42)
                .collect::<String>()
                .split_ascii_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();
            let mut matches = 0;
            for card_number in card_numbers {
                if winning_numbers.contains(&card_number) {
                    matches += 1;
                }
            }
            matches
        })
        .collect();

    let mut card_counts: Vec<u32> = vec![1; card_matches.len()];

    for i in 0..card_matches.len() {
        let matches = card_matches[i];
        let count = card_counts[i];
        for j in 1..=matches {
            card_counts[i + j as usize] += count;
        }
    }

    println!("{}", card_counts.iter().sum::<u32>());
}
