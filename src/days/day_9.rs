use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_9.txt").expect("Error reading the file")
}

pub fn first() {
    let sequences: Vec<Vec<i64>> = read_file()
        .split_terminator('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut acc = 0;
    for sequence in sequences {
        let mut diffs = vec![sequence.clone()];
        while diffs.last().unwrap().iter().any(|&x| x != 0) {
            let previous = diffs.last().unwrap();
            let mut new_diff = Vec::new();
            for i in 1..previous.len() {
                new_diff.push(previous[i] - previous[i - 1]);
            }
            diffs.push(new_diff.clone());
        }

        acc += diffs.iter().map(|x| x.last().unwrap()).sum::<i64>();
    }

    println!("{}", acc);
}

pub fn second() {
    let sequences: Vec<Vec<i64>> = read_file()
        .split_terminator('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut acc = 0;
    for sequence in sequences {
        let mut diffs = vec![sequence.clone()];
        while diffs.last().unwrap().iter().any(|&x| x != 0) {
            let previous = diffs.last().unwrap();
            let mut new_diff = Vec::new();
            for i in 1..previous.len() {
                new_diff.push(previous[i] - previous[i - 1]);
            }
            diffs.push(new_diff.clone());
        }

        acc += diffs
            .iter()
            .map(|x| x[0])
            .rev()
            .skip(1)
            .fold(0, |acc, x| x - acc);
    }

    println!("{}", acc);
}
