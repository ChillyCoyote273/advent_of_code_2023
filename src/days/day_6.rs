use regex::Regex;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_6.txt").expect("Error reading the file")
}

pub fn first() {
    let file = read_file();
    let re = Regex::new(r"(?m)(\d+)").unwrap();
    let numbers = re
        .find_iter(&file)
        .map(|x| x.as_str().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let times = &numbers[0..4];
    let distances = &numbers[4..8];
    let mut acc = 1;
    for i in 0..4 {
        let time = times[i];
        let distance = distances[i];

        let lower = (time / 2.0 - (time * time - 4.0 * distance).sqrt() / 2.0).ceil() as u64;
        let upper = (time / 2.0 + (time * time - 4.0 * distance).sqrt() / 2.0).floor() as u64;

        acc *= upper - lower + 1;
    }

    println!("{}", acc);
}

pub fn second() {
    let file = read_file();
    let re = Regex::new(r"(?m)^\w+:\s+(.+)$").unwrap();
    let numbers = re
        .find_iter(&file)
        .map(|x| {
            x.as_str()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        })
        .collect::<Vec<f64>>();

    let time = numbers[0];
    let distance = numbers[1];

    let lower = (time / 2.0 - (time * time - 4.0 * distance).sqrt() / 2.0).ceil() as u64;
    let upper = (time / 2.0 + (time * time - 4.0 * distance).sqrt() / 2.0).floor() as u64;

    println!("{}", upper - lower + 1);
}
