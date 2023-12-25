use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_16.txt").expect("Error reading the file")
}

pub fn first() {
    let grid = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut mask = vec![vec![false; grid[0].len()]; grid.len()];
    let mut splitter_mask = vec![vec![false; grid[0].len()]; grid.len()];

    let mut beams: Vec<((i32, i32), (i32, i32))> = vec![((0, 0), (0, 1))];
    while let Some(&beam) = beams.last() {
        match grid[beam.0 .0 as usize][beam.0 .1 as usize] {
            '.' => {
                mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                beams.pop();
                let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                    beams.push(new_beam);
                }
            }
            '/' => {
                let new_direction = match beam.1 {
                    (1, 0) => (0, -1), // down -> left
                    (0, 1) => (-1, 0), // right -> up
                    (-1, 0) => (0, 1), // up -> right
                    (0, -1) => (1, 0), // left -> down
                    _ => panic!("Unknown direction"),
                };
                mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                beams.pop();
                let new_beam = (
                    (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                    new_direction,
                );
                if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                    beams.push(new_beam);
                }
            }
            '\\' => {
                let new_direction = match beam.1 {
                    (1, 0) => (0, 1),   // down -> right
                    (0, 1) => (1, 0),   // right -> down
                    (-1, 0) => (0, -1), // up -> left
                    (0, -1) => (-1, 0), // left -> up
                    _ => panic!("Unknown direction"),
                };
                mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                beams.pop();
                let new_beam = (
                    (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                    new_direction,
                );
                if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                    beams.push(new_beam);
                }
            }
            '-' => {
                if beam.1 .0 == 0 {
                    mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    beams.pop();
                    let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }
                } else if splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] {
                    beams.pop();
                } else {
                    splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    beams.pop();

                    let new_direction = (0, 1);
                    let new_beam = (
                        (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                        new_direction,
                    );
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }

                    let new_direction = (0, -1);
                    let new_beam = (
                        (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                        new_direction,
                    );
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }
                }
            }
            '|' => {
                if beam.1 .1 == 0 {
                    mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    beams.pop();
                    let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }
                } else if splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] {
                    beams.pop();
                } else {
                    splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                    beams.pop();

                    let new_direction = (1, 0);
                    let new_beam = (
                        (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                        new_direction,
                    );
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }

                    let new_direction = (-1, 0);
                    let new_beam = (
                        (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                        new_direction,
                    );
                    if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                        beams.push(new_beam);
                    }
                }
            }
            _ => panic!("Unknown character"),
        }
    }

    let count = mask
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>();

    println!("{}", count);
}

pub fn second() {
    let grid = read_file()
        .split_terminator('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_count = (0..110)
        .flat_map(|x| {
            [
                ((x, 0), (0, 1)),
                ((0, x), (1, 0)),
                ((x, 109), (0, -1)),
                ((109, x), (-1, 0)),
            ]
        })
        .map(|entry| {
            let mut mask = vec![vec![false; grid[0].len()]; grid.len()];
            let mut splitter_mask = vec![vec![false; grid[0].len()]; grid.len()];

            let mut beams: Vec<((i32, i32), (i32, i32))> = vec![entry];
            while let Some(&beam) = beams.last() {
                match grid[beam.0 .0 as usize][beam.0 .1 as usize] {
                    '.' => {
                        mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                        beams.pop();
                        let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                        if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                            beams.push(new_beam);
                        }
                    }
                    '/' => {
                        let new_direction = match beam.1 {
                            (1, 0) => (0, -1), // down -> left
                            (0, 1) => (-1, 0), // right -> up
                            (-1, 0) => (0, 1), // up -> right
                            (0, -1) => (1, 0), // left -> down
                            _ => panic!("Unknown direction"),
                        };
                        mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                        beams.pop();
                        let new_beam = (
                            (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                            new_direction,
                        );
                        if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                            beams.push(new_beam);
                        }
                    }
                    '\\' => {
                        let new_direction = match beam.1 {
                            (1, 0) => (0, 1),   // down -> right
                            (0, 1) => (1, 0),   // right -> down
                            (-1, 0) => (0, -1), // up -> left
                            (0, -1) => (-1, 0), // left -> up
                            _ => panic!("Unknown direction"),
                        };
                        mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                        beams.pop();
                        let new_beam = (
                            (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                            new_direction,
                        );
                        if (0..110).contains(&new_beam.0 .0) && (0..110).contains(&new_beam.0 .1) {
                            beams.push(new_beam);
                        }
                    }
                    '-' => {
                        if beam.1 .0 == 0 {
                            mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            beams.pop();
                            let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }
                        } else if splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] {
                            beams.pop();
                        } else {
                            splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            beams.pop();

                            let new_direction = (0, 1);
                            let new_beam = (
                                (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                                new_direction,
                            );
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }

                            let new_direction = (0, -1);
                            let new_beam = (
                                (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                                new_direction,
                            );
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }
                        }
                    }
                    '|' => {
                        if beam.1 .1 == 0 {
                            mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            beams.pop();
                            let new_beam = ((beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1), beam.1);
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }
                        } else if splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] {
                            beams.pop();
                        } else {
                            splitter_mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            mask[beam.0 .0 as usize][beam.0 .1 as usize] = true;
                            beams.pop();

                            let new_direction = (1, 0);
                            let new_beam = (
                                (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                                new_direction,
                            );
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }

                            let new_direction = (-1, 0);
                            let new_beam = (
                                (beam.0 .0 + new_direction.0, beam.0 .1 + new_direction.1),
                                new_direction,
                            );
                            if (0..110).contains(&new_beam.0 .0)
                                && (0..110).contains(&new_beam.0 .1)
                            {
                                beams.push(new_beam);
                            }
                        }
                    }
                    _ => panic!("Unknown character"),
                }
            }

            mask.iter()
                .map(|row| row.iter().filter(|&&x| x).count())
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!("{}", max_count);
}
