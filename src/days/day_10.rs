use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_10.txt").expect("Error reading the file")
}

pub fn first() {
    let map: Vec<Vec<char>> = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut position = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .next()
        .unwrap();

    let mut direction = Down;
    let mut count = 0;

    loop {
        match direction {
            Up => position.1 -= 1,
            Down => position.1 += 1,
            Left => position.0 -= 1,
            Right => position.0 += 1,
        }

        match map[position.1][position.0] {
            '-' => (),
            '|' => (),
            'L' => {
                direction = match direction {
                    Down => Right,
                    Left => Up,
                    _ => panic!(),
                };
            }
            'J' => {
                direction = match direction {
                    Down => Left,
                    Right => Up,
                    _ => panic!(),
                };
            }
            '7' => {
                direction = match direction {
                    Up => Left,
                    Right => Down,
                    _ => panic!(),
                };
            }
            'F' => {
                direction = match direction {
                    Up => Right,
                    Left => Down,
                    _ => panic!(),
                };
            }
            'S' => {
                count += 1;
                break;
            }
            _ => panic!(),
        }

        count += 1;
    }

    println!("{}", count / 2);
}

pub fn second() {
    let mut map: Vec<Vec<char>> = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut position = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .next()
        .unwrap();

    let mut direction = Down;
    let initial = position;

    let mut loop_mask = vec![vec![false; map[0].len()]; map.len()];

    loop {
        loop_mask[position.1][position.0] = true;

        match direction {
            Up => position.1 -= 1,
            Down => position.1 += 1,
            Left => position.0 -= 1,
            Right => position.0 += 1,
        }

        match map[position.1][position.0] {
            '-' => (),
            '|' => (),
            'L' => {
                direction = match direction {
                    Down => Right,
                    Left => Up,
                    _ => panic!(),
                };
            }
            'J' => {
                direction = match direction {
                    Down => Left,
                    Right => Up,
                    _ => panic!(),
                };
            }
            '7' => {
                direction = match direction {
                    Up => Left,
                    Right => Down,
                    _ => panic!(),
                };
            }
            'F' => {
                direction = match direction {
                    Up => Right,
                    Left => Down,
                    _ => panic!(),
                };
            }
            'S' => {
                break;
            }
            _ => panic!(),
        }
    }

    map[initial.1][initial.0] = '7';

    let mut count = 0;
    let mut inside = false;
    for i in 0..loop_mask.len() {
        let mut j = 0;
        while (0..loop_mask[0].len()).contains(&j) {
            if loop_mask[i][j] {
                match map[i][j] {
                    '|' => {
                        inside = !inside;
                    }
                    'L' => {
                        j += 1;
                        while (0..loop_mask[0].len()).contains(&j) && map[i][j] == '-' {
                            j += 1;
                        }
                        if map[i][j] == '7' {
                            inside = !inside;
                        }
                    }
                    'F' => {
                        j += 1;
                        while (0..loop_mask[0].len()).contains(&j) && map[i][j] == '-' {
                            j += 1;
                        }
                        if map[i][j] == 'J' {
                            inside = !inside;
                        }
                    }
                    _ => panic!("Found {}, at ({}, {})", map[i][j], i, j),
                }
            } else if inside {
                count += 1;
            }
            j += 1;
        }
    }

    println!("{}", count);
}
