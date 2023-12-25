use itertools::Itertools;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_15.txt").expect("Error reading the file")
}

enum Step {
    Replace(String, u64),
    Remove(String),
}

impl Step {
    pub fn new(s: &str) -> Step {
        if s.contains('=') {
            let mut split = s.split('=');
            let first = split.next().unwrap().trim();
            let second = split.next().unwrap().trim();
            Step::Replace(first.to_string(), second.parse().unwrap())
        } else {
            let mut chars = s.chars().collect_vec();
            chars.pop();
            Step::Remove(chars.iter().collect::<String>())
        }
    }
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, x| ((acc + x) * 17) % 256)
}

pub fn first() {
    let sum: u64 = read_file()
        .split_terminator(',')
        .map(|x| {
            x.chars()
                .map(|c| c as u64)
                .fold(0, |acc, x| ((acc + x) * 17) % 256)
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {
    let steps = read_file()
        .split_terminator(',')
        .map(Step::new)
        .collect_vec();

    let mut boxes: Vec<Vec<(String, u64)>> = vec![Vec::new(); 256];

    for step in steps {
        match step {
            Step::Replace(s, n) => {
                let hash = hash(&s);
                if let Some((idx, ..)) = boxes[hash].iter().find_position(|x| x.0 == s) {
                    boxes[hash][idx] = (s, n);
                } else {
                    boxes[hash].push((s, n));
                }
            }
            Step::Remove(s) => {
                let hash = hash(&s);
                if let Some((idx, ..)) = boxes[hash].iter().find_position(|x| x.0 == s) {
                    boxes[hash].remove(idx);
                }
            }
        };
    }

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, &(_, n))| (i + 1) * (j + 1) * n as usize)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", sum);
}
