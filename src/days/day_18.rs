use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_18.txt").expect("Error reading the file")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn first() {
    let input = read_file()
        .split_terminator('\n')
        .map(|line| {
            let line = line.split_terminator(' ').collect::<Vec<&str>>();
            let direction = match line[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = line[1].parse::<u32>().unwrap();
            (direction, distance)
        })
        .collect_vec();

    let mut current_tile = (0, 0);
    let mut dug = HashSet::new();
    dug.insert(current_tile);

    for (direction, distance) in input.clone() {
        let delta = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        for _ in 0..distance {
            current_tile.0 += delta.0;
            current_tile.1 += delta.1;
            dug.insert(current_tile);
        }
    }

    let min_r = dug.iter().map(|(x, _)| x).min().unwrap();
    let max_r = dug.iter().map(|(x, _)| x).max().unwrap();
    let min_c = dug.iter().map(|(_, y)| y).min().unwrap();
    let max_c = dug.iter().map(|(_, y)| y).max().unwrap();

    let mut map = vec![vec!['.'; (max_c - min_c + 1) as usize]; (max_r - min_r + 1) as usize];

    current_tile = (-min_r, -min_c);

    let mut previous_direction = Direction::Up;

    for (direction, distance) in input.clone() {
        let delta = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let symbol = match direction {
            Direction::Up if previous_direction == Direction::Left => 'L',
            Direction::Up if previous_direction == Direction::Right => 'J',
            Direction::Down if previous_direction == Direction::Left => 'F',
            Direction::Down if previous_direction == Direction::Right => '7',
            Direction::Left if previous_direction == Direction::Up => '7',
            Direction::Left if previous_direction == Direction::Down => 'J',
            Direction::Right if previous_direction == Direction::Up => 'F',
            Direction::Right if previous_direction == Direction::Down => 'L',
            _ => panic!(
                "Invalid direction: Current: {:?}, Previous: {:?}",
                direction, previous_direction
            ),
        };

        previous_direction = direction;

        map[current_tile.0 as usize][current_tile.1 as usize] = symbol;

        if direction == Direction::Up || direction == Direction::Down {
            for _ in 0..distance {
                current_tile.0 += delta.0;
                current_tile.1 += delta.1;
                map[current_tile.0 as usize][current_tile.1 as usize] = '|';
            }
        } else {
            for _ in 0..distance {
                current_tile.0 += delta.0;
                current_tile.1 += delta.1;
                map[current_tile.0 as usize][current_tile.1 as usize] = '-';
            }
        }
    }

    map[current_tile.0 as usize][current_tile.1 as usize] = match input[0].0 {
        Direction::Up if previous_direction == Direction::Left => 'L',
        Direction::Up if previous_direction == Direction::Right => 'J',
        Direction::Down if previous_direction == Direction::Left => 'F',
        Direction::Down if previous_direction == Direction::Right => '7',
        Direction::Left if previous_direction == Direction::Up => '7',
        Direction::Left if previous_direction == Direction::Down => 'J',
        Direction::Right if previous_direction == Direction::Up => 'F',
        Direction::Right if previous_direction == Direction::Down => 'L',
        _ => panic!("Invalid direction",),
    };

    let mut inside = false;
    for i in 0..map.len() {
        let mut j = 0;
        while (0..map[0].len()).contains(&j) {
            if map[i][j] != '.' {
                match map[i][j] {
                    '|' => {
                        inside = !inside;
                    }
                    'L' => {
                        j += 1;
                        while (0..map[0].len()).contains(&j) && map[i][j] == '-' {
                            j += 1;
                        }
                        if map[i][j] == '7' {
                            inside = !inside;
                        }
                    }
                    'F' => {
                        j += 1;
                        while (0..map[0].len()).contains(&j) && map[i][j] == '-' {
                            j += 1;
                        }
                        if map[i][j] == 'J' {
                            inside = !inside;
                        }
                    }
                    _ => panic!("Found {}, at ({}, {})", map[i][j], i, j),
                }
            } else if inside {
                map[i][j] = '#';
            }
            j += 1;
        }
    }

    map[-min_r as usize][-min_c as usize] = 'X';

    println!("{}", map.iter().flatten().filter(|&&c| c != '.').count());
}

pub fn second() {
    let input = read_file()
        .split_terminator('\n')
        .map(|line| {
            let number = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .chars()
                .skip(2)
                .take(6);
            let distance =
                u64::from_str_radix(&number.clone().take(5).collect::<String>(), 16).unwrap();
            let direction = match number.last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction"),
            };
            (direction, distance)
        })
        .collect_vec();

    let mut point_a = (0, 0);
    let mut verticals_a = HashSet::new();

    let mut point_b = (0, 0);
    let mut verticals_b = HashSet::new();

    for (&(previous_direction, _), &(direction, distance), &(next_direction, _)) in
        [*input.last().unwrap()]
            .iter()
            .chain(input.clone().iter())
            .chain([*input.first().unwrap()].iter())
            .tuple_windows::<(_, _, _)>()
    {
        let delta = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let distance_modifier = if previous_direction == next_direction {
            0
        } else {
            match (previous_direction, direction, next_direction) {
                (Direction::Up, Direction::Right, Direction::Down)
                | (Direction::Right, Direction::Down, Direction::Left)
                | (Direction::Down, Direction::Left, Direction::Up)
                | (Direction::Left, Direction::Up, Direction::Right) => 1,
                _ => -1,
            }
        };

        let next_point_a = (
            point_a.0 + delta.0 * (distance as i64 + distance_modifier),
            point_a.1 + delta.1 * (distance as i64 + distance_modifier),
        );

        if direction == Direction::Up {
            verticals_a.insert((point_a.1, next_point_a.0..=point_a.0));
        } else if direction == Direction::Down {
            verticals_a.insert((point_a.1, point_a.0..=next_point_a.0));
        }

        point_a = next_point_a;

        let next_point_b = (
            point_b.0 + delta.0 * (distance as i64 - distance_modifier),
            point_b.1 + delta.1 * (distance as i64 - distance_modifier),
        );

        if direction == Direction::Up {
            verticals_b.insert((point_b.1, next_point_b.0..=point_b.0));
        } else if direction == Direction::Down {
            verticals_b.insert((point_b.1, point_b.0..=next_point_b.0));
        }

        point_b = next_point_b;
    }

    let volume_a: u64 = verticals_a
        .clone()
        .iter()
        .flat_map(|(_, range)| [*range.start(), *range.end()])
        .sorted_unstable()
        .dedup()
        .tuple_windows()
        .map(|(start, end)| {
            verticals_a
                .iter()
                .filter_map(|(c, r)| {
                    if r.contains(&start) && r.contains(&end) {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .sorted_unstable()
                .chunks(2)
                .into_iter()
                .map(|chunk| {
                    let chunk = chunk.collect_vec();
                    (chunk[1] - chunk[0]) as u64
                })
                .sum::<u64>()
                * (end - start) as u64
        })
        .sum();

    let volume_b: u64 = verticals_b
        .clone()
        .iter()
        .flat_map(|(_, range)| [*range.start(), *range.end()])
        .sorted_unstable()
        .dedup()
        .tuple_windows()
        .map(|(start, end)| {
            verticals_b
                .iter()
                .filter_map(|(c, r)| {
                    if r.contains(&start) && r.contains(&end) {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .sorted_unstable()
                .chunks(2)
                .into_iter()
                .map(|chunk| {
                    let chunk = chunk.collect_vec();
                    (chunk[1] - chunk[0]) as u64
                })
                .sum::<u64>()
                * (end - start) as u64
        })
        .sum();

    println!("{}", u64::max(volume_a, volume_b));
}
