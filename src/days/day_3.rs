use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_3.txt").expect("Error reading the file")
}

pub fn first() {
    let grid: Vec<Vec<char>> = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();
    let mut mask = vec![vec![false; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' || grid[i][j].is_digit(10) {
                continue;
            }

            for k in -1..=1 {
                for l in -1..=1 {
                    let x = i as i32 + k;
                    let y = j as i32 + l;
                    if (0..grid.len() as i32).contains(&x) && (0..grid[i].len() as i32).contains(&y)
                    {
                        mask[x as usize][y as usize] = true;
                    }
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..mask.len() {
        for j in 0..mask[i].len() {
            if mask[i][j] && grid[i][j].is_digit(10) {
                let mut start = j as i32;
                while (0..mask[i].len() as i32).contains(&start)
                    && grid[i][start as usize].is_digit(10)
                {
                    start -= 1;
                }
                start += 1;
                let mut end = j;
                while (0..mask[i].len()).contains(&end) && grid[i][end as usize].is_digit(10) {
                    end += 1;
                }
                let result: u32 = grid[i][start as usize..end as usize]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                count += result;
                for k in start as usize..end as usize {
                    mask[i][k] = false;
                }
            }
        }
    }

    println!("{}", count);
}

fn search(grid: &Vec<Vec<char>>, i: usize, j: i32) -> u32 {
    let mut start = j;
    while (0..grid[i].len() as i32).contains(&start) && grid[i][start as usize].is_digit(10) {
        start -= 1;
    }
    start += 1;
    let mut end = j;
    while (0..grid[i].len() as i32).contains(&end) && grid[i][end as usize].is_digit(10) {
        end += 1;
    }
    grid[i][start as usize..end as usize]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn second() {
    let grid: Vec<Vec<char>> = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            if grid[i as usize][j as usize] == '*' {
                let mut adjacent = 0;
                let mut gear = 1;
                for k in [-1, 1] {
                    if (0..grid.len() as i32).contains(&(i + k)) {
                        if grid[(i + k) as usize][j as usize].is_digit(10) {
                            adjacent += 1;
                            gear *= search(&grid, (i + k) as usize, j);
                        } else {
                            if (0..grid[(i + k) as usize].len() as i32).contains(&(j - 1))
                                && grid[(i + k) as usize][(j - 1) as usize].is_digit(10)
                            {
                                adjacent += 1;
                                gear *= search(&grid, (i + k) as usize, j - 1);
                            }
                            if (0..grid[(i + k) as usize].len() as i32).contains(&(j + 1))
                                && grid[(i + k) as usize][(j + 1) as usize].is_digit(10)
                            {
                                adjacent += 1;
                                gear *= search(&grid, (i + k) as usize, j + 1);
                            }
                        }
                    }
                }
                if (0..grid[i as usize].len() as i32).contains(&(j - 1))
                    && grid[i as usize][(j - 1) as usize].is_digit(10)
                {
                    adjacent += 1;
                    gear *= search(&grid, i as usize, j - 1);
                }
                if (0..grid[i as usize].len() as i32).contains(&(j + 1))
                    && grid[i as usize][(j + 1) as usize].is_digit(10)
                {
                    adjacent += 1;
                    gear *= search(&grid, i as usize, j + 1);
                }
                if adjacent == 2 {
                    count += gear;
                }
            }
        }
    }

    println!("{}", count);
}
