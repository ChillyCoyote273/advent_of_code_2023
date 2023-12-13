use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_11.txt").expect("Error reading the file")
}

pub fn first() {
    let mut universe = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let empty = vec!['.'; universe[0].len()];
    let mut i = 0;
    while i < universe.len() {
        if universe[i] == empty {
            universe.insert(i, empty.clone());
            i += 1;
        }
        i += 1;
    }
    let mut universe_trans = vec![vec!['.'; universe.len()]; universe[0].len()];
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            universe_trans[j][i] = universe[i][j];
        }
    }
    universe = universe_trans;
    let empty = vec!['.'; universe[0].len()];
    let mut i = 0;
    while i < universe.len() {
        if universe[i] == empty {
            universe.insert(i, empty.clone());
            i += 1;
        }
        i += 1;
    }
    let mut stars: Vec<(i64, i64)> = Vec::new();
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                stars.push((i as i64, j as i64));
            }
        }
    }

    let mut acc = 0;
    for i in 0..stars.len() {
        for j in 0..i {
            acc += (stars[i].0 - stars[j].0).abs() + (stars[i].1 - stars[j].1).abs();
        }
    }

    println!("{}", acc);
}

pub fn second() {
    let universe = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut row_gaps = Vec::new();
    let empty = vec!['.'; universe[0].len()];
    let mut i = 0;
    while i < universe.len() {
        if universe[i] == empty {
            row_gaps.push(i);
        }
        i += 1;
    }
    let mut col_gaps = Vec::new();
    for i in 0..universe[0].len() {
        let mut empty = true;
        for j in 0..universe.len() {
            if universe[j][i] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            col_gaps.push(i);
        }
    }

    let mut stars: Vec<(i64, i64)> = Vec::new();
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                let mut row = i;
                let mut col = j;
                let mut gaps = 0;
                for gap in &row_gaps {
                    if *gap < row {
                        gaps += 1;
                    }
                }
                row += gaps * 999_999;
                let mut gaps = 0;
                for gap in &col_gaps {
                    if *gap < col {
                        gaps += 1;
                    }
                }
                col += gaps * 999_999;
                stars.push((row as i64, col as i64));
            }
        }
    }

    let mut acc = 0;
    for i in 0..stars.len() {
        for j in 0..i {
            acc += (stars[i].0 - stars[j].0).abs() + (stars[i].1 - stars[j].1).abs();
        }
    }

    println!("{}", acc);
}
