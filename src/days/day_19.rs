use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Range, RangeInclusive};

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_19.txt").expect("Error reading the file")
}

pub fn first() {
    let input = read_file()
        .split_terminator("\n\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let rules = input[0]
        .split_terminator('\n')
        .map(|line| {
            let name = line.chars().take_while(|&c| c != '{').collect::<String>();
            let rules = line
                .chars()
                .skip_while(|&c| c != '{')
                .skip(1)
                .take_while(|&c| c != '}')
                .collect::<String>()
                .split_terminator(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let index = match rule.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!("Invalid rule: {}", rule),
                        };
                        let symbol = rule.chars().nth(1).unwrap();
                        let value = rule
                            .chars()
                            .skip(2)
                            .take_while(|&c| c != ':')
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        let destination = rule.chars().skip_while(|&c| c != ':').skip(1).collect();
                        let range = match symbol {
                            '>' => (value + 1)..usize::MAX,
                            '<' => 0..value,
                            _ => panic!("Invalid symbol"),
                        };
                        (index, range, destination)
                    } else {
                        (0, 0..usize::MAX, rule.to_string())
                    }
                })
                .collect::<Vec<(usize, Range<usize>, String)>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = input[1]
        .split_terminator('\n')
        .map(|line| {
            line.split_terminator(',')
                .map(|part| {
                    part.chars()
                        .skip_while(|&c| !c.is_ascii_digit())
                        .take_while(|&c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut acc = 0;
    for part in parts {
        let mut current_rule = "in".to_string();
        while &current_rule != "A" && &current_rule != "R" {
            for (index, range, destination) in &rules[&current_rule] {
                if range.contains(&part[*index]) {
                    current_rule = destination.to_string();
                    break;
                }
            }
        }
        if &current_rule == "A" {
            acc += part.iter().sum::<usize>();
        }
    }

    println!("The sum of the part values is {}", acc);
}

fn union(
    a: Vec<RangeInclusive<usize>>,
    b: Vec<RangeInclusive<usize>>,
) -> Vec<RangeInclusive<usize>> {
    let points = a
        .iter()
        .flat_map(|range| [*range.start(), *range.end()])
        .chain(b.iter().flat_map(|range| [*range.start(), *range.end()]))
        .sorted_unstable()
        .dedup()
        .collect::<Vec<_>>();

    let mut result = Vec::new();

    'outer: for (&first, &last) in points.iter().zip(points.iter().skip(1)) {
        let mid = (first + last) / 2;
        for range in a.iter() {
            if range.contains(&mid) {
                result.push(first..=last);
                continue 'outer;
            }
        }
        for range in b.iter() {
            if range.contains(&mid) {
                result.push(first..=last);
                continue 'outer;
            }
        }
    }

    let mut i = 0;
    while i < result.len() - 1 {
        if result[i].end() == result[i + 1].start() {
            result[i] = (*result[i].start())..=(*result[i + 1].end());
            result.remove(i + 1);
        } else {
            i += 1;
        }
    }

    result
}

fn subtract(
    a: &Vec<RangeInclusive<usize>>,
    b: RangeInclusive<usize>,
) -> Vec<RangeInclusive<usize>> {
    let mut result = a.clone();
    let mut i = 0;
    while i < result.len() {
        if b.contains(result[i].start()) && b.contains(result[i].end()) {
            result.remove(i);
            continue;
        }

        if b.contains(result[i].start()) {
            result[i] = (*b.end() + 1)..=*result[i].end();
        } else if b.contains(result[i].end()) {
            result[i] = *result[i].start()..=(*b.start() - 1);
        } else if result[i].contains(b.start()) && result[i].contains(b.end()) {
            result.push(*result[i].start()..=(*b.start() - 1));
            result[i] = (*b.end() + 1)..=*result[i].end();
        }

        i += 1;
    }

    result
}

type RuleSet = HashMap<String, Vec<(usize, RangeInclusive<usize>, String)>>;
type RangeSet = Vec<Vec<RangeInclusive<usize>>>;
fn get_ranges(rules: &RuleSet, ranges: &RangeSet, current_rule: &str) -> usize {
    if current_rule == "R" {
        return 0;
    }
    if current_rule == "A" {
        return ranges
            .iter()
            .map(|range| -> usize {
                range
                    .iter()
                    .map(|range| range.end() - range.start() + 1)
                    .sum()
            })
            .product();
    }

    let mut result = 0;
    let mut current_ranges = ranges.clone();

    for (index, range, destination) in &rules[current_rule] {
        if range != &(1..=4000) {
            let invert_range = if *range.start() == 1 {
                (*range.end() + 1)..=4000
            } else {
                1..=(*range.start() - 1)
            };
            let mut recurse_ranges = current_ranges.clone();
            recurse_ranges[*index] = subtract(&recurse_ranges[*index], invert_range.clone());
            result += get_ranges(rules, &recurse_ranges, destination);
            current_ranges[*index] = subtract(&current_ranges[*index], range.clone());
        } else {
            result += get_ranges(rules, &current_ranges, destination);
        }
    }

    result
}

pub fn second() {
    let input = read_file()
        .split_terminator("\n\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let rules = input[0]
        .split_terminator('\n')
        .map(|line| {
            let name = line.chars().take_while(|&c| c != '{').collect::<String>();
            let rules = line
                .chars()
                .skip_while(|&c| c != '{')
                .skip(1)
                .take_while(|&c| c != '}')
                .collect::<String>()
                .split_terminator(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let index = match rule.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!("Invalid rule: {}", rule),
                        };
                        let symbol = rule.chars().nth(1).unwrap();
                        let value = rule
                            .chars()
                            .skip(2)
                            .take_while(|&c| c != ':')
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        let destination = rule.chars().skip_while(|&c| c != ':').skip(1).collect();
                        let range = match symbol {
                            '>' => (value + 1)..=4000,
                            '<' => 1..=(value - 1),
                            _ => panic!("Invalid symbol"),
                        };
                        (index, range, destination)
                    } else {
                        (0, 1..=4000, rule.to_string())
                    }
                })
                .collect::<Vec<(usize, RangeInclusive<usize>, String)>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let initial_ranges = vec![vec![1..=4000]; 4];
    let possibility_count = get_ranges(&rules, &initial_ranges, "in");
    println!("The number of possibilities is {}", possibility_count)
}
