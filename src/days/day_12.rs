use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_12.txt").expect("Error reading the file")
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
struct Key {
    map: Vec<Option<bool>>,
    encoding: Vec<u64>,
    running: bool,
}

fn count_possible_arrangements_recursive(
    map: Vec<Option<bool>>,
    encoding: Vec<u64>,
    running: bool,
    min_diff: i64,
    max_diff: i64,
    memoization_table: &mut HashMap<Key, u64>,
) -> u64 {
    let key = Key {
        map: map.clone(),
        encoding: encoding.clone(),
        running,
    };
    if let Some(&value) = memoization_table.get(&key) {
        return value;
    }
    if min_diff < 0 || max_diff < 0 {
        memoization_table.insert(key, 0);
        return 0;
    }
    if let Some(&c) = map.last() {
        match c {
            Some(true) => {
                // '#'
                if encoding.is_empty() || encoding.last() == Some(&0) {
                    memoization_table.insert(key, 0);
                    return 0;
                }
                let mut new_map = map.clone();
                new_map.pop();
                let mut new_encoding = encoding.clone();
                if let Some(n) = new_encoding.last_mut() {
                    *n -= 1;
                }
                let ret = count_possible_arrangements_recursive(
                    new_map,
                    new_encoding,
                    true,
                    min_diff,
                    max_diff,
                    memoization_table,
                );
                memoization_table.insert(key, ret);
                ret
            }
            Some(false) => {
                // '.'
                if running && encoding.last() != Some(&0) {
                    memoization_table.insert(key, 0);
                    return 0;
                }
                let mut new_map = map.clone();
                new_map.pop();
                let mut new_encoding = encoding.clone();
                if !new_encoding.is_empty() && new_encoding.last() == Some(&0) {
                    new_encoding.pop();
                }
                let ret = count_possible_arrangements_recursive(
                    new_map,
                    new_encoding,
                    false,
                    min_diff,
                    max_diff,
                    memoization_table,
                ); // no change in window
                memoization_table.insert(key, ret);
                ret
            }
            None => {
                // '?'
                let mut broken = map.clone();
                broken.pop();
                broken.push(Some(true));
                let mut not_broken = map.clone();
                not_broken.pop();
                not_broken.push(Some(false));
                let ret = count_possible_arrangements_recursive(
                    broken,
                    encoding.clone(),
                    running,
                    min_diff - 1,
                    max_diff,
                    memoization_table,
                ) // min_diff -= 1;
                    + count_possible_arrangements_recursive(
                    not_broken,
                    encoding.clone(),
                    running,
                    min_diff,
                    max_diff - 1,
                    memoization_table,
                );
                memoization_table.insert(key, ret);
                ret
                // max_diff -= 1;
            }
        }
    } else {
        // empty map
        match encoding.last() {
            Some(&n) => {
                if n == 0 && encoding.len() == 1 {
                    memoization_table.insert(key, 1);
                    // empty map and encoding
                    1
                } else {
                    memoization_table.insert(key, 0);
                    // empty map and non-empty encoding
                    0
                }
            }
            None => {
                memoization_table.insert(key, 1);
                1
            } // empty map and encoding
        }
    }
}

pub fn first() {
    let sum: u64 = read_file()
        .split_terminator('\n')
        .map(|line| {
            let line_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let map = line_parts[0]
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => None,
                })
                .collect::<Vec<Option<bool>>>();
            let encoding = line_parts[1]
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let empty_spaces = map.iter().filter(|c| c.is_none()).count() as u64;
            let broken_spaces = map.iter().filter(|&&c| c == Some(true)).count() as u64;
            let required_broken = encoding.iter().sum::<u64>();

            let min_filled = broken_spaces;
            let max_filled = broken_spaces + empty_spaces;

            let min_diff = required_broken - min_filled;
            let max_diff = max_filled - required_broken;

            count_possible_arrangements_recursive(
                map.iter().rev().cloned().collect::<Vec<_>>(),
                encoding.iter().rev().cloned().collect::<Vec<_>>(),
                false,
                min_diff as i64,
                max_diff as i64,
                &mut HashMap::new(),
            )
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {
    let sum: u64 = read_file()
        .split_terminator('\n')
        .collect_vec()
        .par_iter()
        .map(|line| {
            let line_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let map_fragment = line_parts[0]
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => None,
                })
                .chain([None]);
            let map: Vec<Option<bool>> = map_fragment
                .clone()
                .cycle()
                .take(map_fragment.count() * 5 - 1)
                .collect();
            let encoding_fragment = line_parts[1].split(',').map(|s| s.parse::<u64>().unwrap());
            let encoding: Vec<u64> = encoding_fragment
                .clone()
                .cycle()
                .take(encoding_fragment.count() * 5)
                .collect();

            let empty_spaces = map.iter().filter(|c| c.is_none()).count() as u64;
            let broken_spaces = map.iter().filter(|&&c| c == Some(true)).count() as u64;
            let required_broken = encoding.iter().sum::<u64>();

            let min_filled = broken_spaces;
            let max_filled = broken_spaces + empty_spaces;

            let min_diff = required_broken - min_filled;
            let max_diff = max_filled - required_broken;

            count_possible_arrangements_recursive(
                map.iter().rev().cloned().collect::<Vec<_>>(),
                encoding.iter().rev().cloned().collect::<Vec<_>>(),
                false,
                min_diff as i64,
                max_diff as i64,
                &mut HashMap::new(),
            )
        })
        .sum();

    println!("{}", sum);
}
