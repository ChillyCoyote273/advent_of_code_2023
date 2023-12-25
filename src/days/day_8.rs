use std::collections::HashMap;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_8.txt").expect("Error reading the file")
}

pub fn first() {
    let file = read_file();
    let input = file.split_terminator("\n\n").collect::<Vec<&str>>();
    let instructions = input[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let nodes = input[1]
        .split_terminator('\n')
        .map(|s| {
            vec![
                s[0..3].to_string(),
                s[7..10].to_string(),
                s[12..15].to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes {
        map.insert(node[0].clone(), (node[1].clone(), node[2].clone()));
    }

    let mut steps = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ" {
        let instruction = instructions[steps % instructions.len()];
        let next = if instruction == 0 {
            map.get(&current).unwrap().0.clone()
        } else {
            map.get(&current).unwrap().1.clone()
        };
        current = next;
        steps += 1;
    }

    println!("{}", steps);
}

pub fn second() {
    let lengths: Vec<usize> = vec![14257, 16409, 15871, 18023, 12643, 19637];

    let mut lcm = 1;
    let mut div = 2;
    let mut lengths = lengths.clone();
    while lengths.iter().any(|l| *l != 1) {
        let mut found = false;
        for length in lengths.iter_mut() {
            if *length % div == 0 {
                *length /= div;
                found = true;
            }
        }
        if found {
            lcm *= div;
        } else {
            div += 1;
        }
    }
    println!("{}", lcm);
}
