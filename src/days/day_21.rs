use itertools::Itertools;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_21.txt").expect("Error reading the file")
}

pub fn first() {
    let mut map = read_file()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Some(false),
                    'S' => Some(true),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let mut mask = vec![vec![false; map[0].len()]; map.len()];
    let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for _ in 0..64 {
        for r in 0..mask.len() {
            for c in 0..mask[0].len() {
                mask[r][c] = map[r][c].unwrap_or(false);
                for (r_offset, c_offset) in offsets {
                    let r_new = r as i32 + r_offset;
                    let c_new = c as i32 + c_offset;
                    if !(0..map.len() as i32).contains(&r_new)
                        || !(0..map[0].len() as i32).contains(&c_new)
                    {
                        continue;
                    }
                    mask[r][c] |= map[r_new as usize][c_new as usize].unwrap_or(false);
                }
            }
        }

        for r in 0..map.len() {
            for c in 0..map[0].len() {
                if let Some(value) = &mut map[r][c] {
                    *value ^= mask[r][c];
                }
            }
        }
    }

    let count = map
        .iter()
        .flatten()
        .filter(|&&value| value == Some(true))
        .count();

    println!("{}", count);
}

pub fn second() {
    let points = [(0.0, 3882.0), (2.0, 95442.0), (4.0, 308770.0)]; // quadratic pattern
    let func = |x: f64| {
        (x - points[0].0) * (x - points[1].0)
            / ((points[2].0 - points[0].0) * (points[2].0 - points[1].0))
            * points[2].1
            + (x - points[2].0) * (x - points[1].0)
                / ((points[0].0 - points[2].0) * (points[0].0 - points[1].0))
                * points[0].1
            + (x - points[0].0) * (x - points[2].0)
                / ((points[1].0 - points[0].0) * (points[1].0 - points[2].0))
                * points[1].1
    };
    println!("{}", func((26501365 / 131) as f64));
}
