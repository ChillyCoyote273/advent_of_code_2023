use itertools::Itertools;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_13.txt").expect("Error reading the file")
}

fn check_mirror_vertical(grid: &Vec<Vec<bool>>, line: usize) -> bool {
    let distance = usize::min(line, grid[0].len() - line);
    let range = (line - distance)..(line + distance);
    let mut new_grid = vec![vec![false; distance * 2]; grid.len()];
    let mut old_grid = vec![vec![false; distance * 2]; grid.len()];
    for i in 0..grid.len() {
        for (j_new, j_old) in range.clone().rev().enumerate() {
            new_grid[i][j_new] = grid[i][j_old];
            old_grid[i][distance * 2 - j_new - 1] = grid[i][j_old];
        }
    }

    new_grid == old_grid
}

fn check_mirror_horizontal(grid: &Vec<Vec<bool>>, line: usize) -> bool {
    let distance = usize::min(line, grid.len() - line);
    let range = (line - distance)..(line + distance);
    let mut new_grid = vec![vec![false; grid[0].len()]; distance * 2];
    let mut old_grid = vec![vec![false; grid[0].len()]; distance * 2];
    for (i_new, i_old) in range.rev().enumerate() {
        for j in 0..grid[0].len() {
            new_grid[i_new][j] = grid[i_old][j];
            old_grid[distance * 2 - i_new - 1][j] = grid[i_old][j];
        }
    }

    new_grid == old_grid
}

fn check_mirror_vertical_smudge(grid: &Vec<Vec<bool>>, line: usize) -> bool {
    let distance = usize::min(line, grid[0].len() - line);
    let range = (line - distance)..(line + distance);
    let mut new_grid = vec![vec![false; distance * 2]; grid.len()];
    let mut old_grid = vec![vec![false; distance * 2]; grid.len()];
    for i in 0..grid.len() {
        for (j_new, j_old) in range.clone().rev().enumerate() {
            new_grid[i][j_new] = grid[i][j_old];
            old_grid[i][distance * 2 - j_new - 1] = grid[i][j_old];
        }
    }

    check_smudge(new_grid, old_grid)
}

fn check_mirror_horizontal_smudge(grid: &Vec<Vec<bool>>, line: usize) -> bool {
    let distance = usize::min(line, grid.len() - line);
    let range = (line - distance)..(line + distance);
    let mut new_grid = vec![vec![false; grid[0].len()]; distance * 2];
    let mut old_grid = vec![vec![false; grid[0].len()]; distance * 2];
    for (i_new, i_old) in range.rev().enumerate() {
        for j in 0..grid[0].len() {
            new_grid[i_new][j] = grid[i_old][j];
            old_grid[distance * 2 - i_new - 1][j] = grid[i_old][j];
        }
    }

    check_smudge(new_grid, old_grid)
}

fn check_smudge(first: Vec<Vec<bool>>, second: Vec<Vec<bool>>) -> bool {
    let mut count = 0;
    for i in 0..first.len() {
        for j in 0..first[0].len() {
            if first[i][j] != second[i][j] {
                count += 1;
            }
        }
    }
    count == 2
}

pub fn first() {
    let answer: usize = read_file()
        .split_terminator("\n\n")
        .map(|grid| -> usize {
            let grid = grid
                .split_terminator('\n')
                .map(|line| line.chars().map(|c| c == '#').collect_vec())
                .collect_vec();
            let vertical_range = 1..grid[0].len();
            let horizontal_range = 1..grid.len();
            100 * horizontal_range
                .filter(|&i| check_mirror_horizontal(&grid, i))
                .sum::<usize>()
                + vertical_range
                    .filter(|&i| check_mirror_vertical(&grid, i))
                    .sum::<usize>()
        })
        .sum();

    println!("{}", answer);
}

pub fn second() {
    let answer: usize = read_file()
        .split_terminator("\n\n")
        .map(|grid| -> usize {
            let grid = grid
                .split_terminator('\n')
                .map(|line| line.chars().map(|c| c == '#').collect_vec())
                .collect_vec();
            let vertical_range = 1..grid[0].len();
            let horizontal_range = 1..grid.len();
            100 * horizontal_range
                .filter(|&i| check_mirror_horizontal_smudge(&grid, i))
                .sum::<usize>()
                + vertical_range
                    .filter(|&i| check_mirror_vertical_smudge(&grid, i))
                    .sum::<usize>()
        })
        .sum();

    println!("{}", answer);
}
