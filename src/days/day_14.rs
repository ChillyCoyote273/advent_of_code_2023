use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_14.txt").expect("Error reading the file")
}

pub fn first() {
    let mut map = read_file()
        .split_terminator('\n')
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    for i in 1..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != 'O' {
                continue;
            }

            let mut roll = 0;
            for k in (0..i).rev() {
                if map[k][j] != '.' {
                    roll = k + 1;
                    break;
                }
            }
            map[i][j] = '.';
            map[roll][j] = 'O';
        }
    }

    let sum = map
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .map(|&y| if y == 'O' { map.len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", sum);
}

fn cycle(map: &mut Vec<Vec<char>>) {
    // North
    for i in 1..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != 'O' {
                continue;
            }

            let mut roll = 0;
            for k in (0..i).rev() {
                if map[k][j] != '.' {
                    roll = k + 1;
                    break;
                }
            }
            map[i][j] = '.';
            map[roll][j] = 'O';
        }
    }

    // West
    for i in 0..map.len() {
        for j in 1..map[i].len() {
            if map[i][j] != 'O' {
                continue;
            }

            let mut roll = 0;
            for k in (0..j).rev() {
                if map[i][k] != '.' {
                    roll = k + 1;
                    break;
                }
            }
            map[i][j] = '.';
            map[i][roll] = 'O';
        }
    }

    // South
    for i in (0..(map.len() - 1)).rev() {
        for j in 0..map[i].len() {
            if map[i][j] != 'O' {
                continue;
            }

            let mut roll = map.len() - 1;
            for k in (i + 1)..map.len() {
                if map[k][j] != '.' {
                    roll = k - 1;
                    break;
                }
            }
            map[i][j] = '.';
            map[roll][j] = 'O';
        }
    }

    // East
    for i in 0..map.len() {
        for j in (0..(map[i].len() - 1)).rev() {
            if map[i][j] != 'O' {
                continue;
            }

            let mut roll = map[i].len() - 1;
            for k in (j + 1)..map[i].len() {
                if map[i][k] != '.' {
                    roll = k - 1;
                    break;
                }
            }
            map[i][j] = '.';
            map[i][roll] = 'O';
        }
    }
}

pub fn second() {
    let mut map = read_file()
        .split_terminator('\n')
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut memoize = HashMap::new();
    let mut counter = 0;

    while !memoize.contains_key(&map) {
        memoize.insert(map.clone(), counter);
        cycle(&mut map);
        counter += 1;
    }

    let cycle_start = memoize.get(&map).unwrap();
    let cycle_length = counter - cycle_start;
    let target = 1_000_000_000;
    let target_step = (target - cycle_start) % cycle_length;
    let target_map_value = cycle_start + target_step;
    let target_map = memoize
        .iter()
        .find(|(_, &v)| v == target_map_value)
        .unwrap()
        .0;

    let sum = target_map
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .map(|&y| if y == 'O' { target_map.len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", sum);
}
