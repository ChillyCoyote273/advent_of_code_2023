use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::vec::IntoIter;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_22.txt").expect("Error reading the file")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    x: u64,
    y: u64,
    z: u64,
    direction: Direction,
    size: u64,
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.split('~').collect::<Vec<&str>>();
        let first = points[0]
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();
        let second = points[1]
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let (x, y, z) = (
            u64::min(first[0], second[0]),
            u64::min(first[1], second[1]),
            u64::min(first[2], second[2]) - 1,
        );
        let (high_x, high_y, high_z) = (
            u64::max(first[0], second[0]),
            u64::max(first[1], second[1]),
            u64::max(first[2], second[2]) - 1,
        );
        let (direction, size) = match (
            first[0] == second[0],
            first[1] == second[1],
            first[2] == second[2],
        ) {
            (true, true, false) => (Direction::Z, high_z - z + 1),
            (true, false, true) => (Direction::Y, high_y - y + 1),
            (false, true, true) => (Direction::X, high_x - x + 1),
            (true, true, true) => (Direction::Z, 1),
            _ => panic!("Invalid block"),
        };
        Ok(Block {
            x,
            y,
            z,
            direction,
            size,
        })
    }
}

struct BlockIterator {
    block: Block,
    current: u64,
}

impl Iterator for BlockIterator {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.block.size {
            let (x, y, z) = match self.block.direction {
                Direction::X => (self.block.x + self.current, self.block.y, self.block.z),
                Direction::Y => (self.block.x, self.block.y + self.current, self.block.z),
                Direction::Z => (self.block.x, self.block.y, self.block.z + self.current),
            };
            self.current += 1;
            Some((x, y, z))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.block.size - self.current;
        (remaining as usize, Some(remaining as usize))
    }
}

impl IntoIterator for Block {
    type Item = (u64, u64, u64);
    type IntoIter = BlockIterator;

    fn into_iter(self) -> Self::IntoIter {
        BlockIterator {
            block: self,
            current: 0,
        }
    }
}

impl Block {
    pub fn max_point(&self) -> (u64, u64, u64) {
        match self.direction {
            Direction::X => (self.x + self.size - 1, self.y, self.z),
            Direction::Y => (self.x, self.y + self.size - 1, self.z),
            Direction::Z => (self.x, self.y, self.z + self.size - 1),
        }
    }
}

pub fn first() {
    let mut blocks = read_file()
        .lines()
        .map(|s| s.parse::<Block>().unwrap())
        .sorted_unstable_by_key(|block| block.z)
        .collect_vec();

    let (max_x, max_y, max_z) = blocks.iter().fold((0, 0, 0), |acc, block| {
        let point = block.max_point();
        (
            u64::max(acc.0, point.0),
            u64::max(acc.1, point.1),
            u64::max(acc.2, point.2),
        )
    });

    let mut grid =
        vec![vec![vec![false; max_z as usize + 1]; max_y as usize + 1]; max_x as usize + 1];

    for block in blocks.clone() {
        for (x, y, z) in block {
            grid[x as usize][y as usize][z as usize] = true;
        }
    }

    for (i, block) in blocks.clone().into_iter().enumerate() {
        let mut height = block.z as i64 - 1;
        if block.direction == Direction::Z {
            while height >= 0 && !grid[block.x as usize][block.y as usize][height as usize] {
                height -= 1;
            }
        } else {
            while height >= 0 {
                let mut new_block = block;
                new_block.z = height as u64;
                if new_block
                    .into_iter()
                    .any(|(x, y, z)| grid[x as usize][y as usize][z as usize])
                {
                    break;
                };
                height -= 1;
            }
        }
        height += 1;
        let mut new_block = block;
        new_block.z = height as u64;
        for (x, y, z) in block {
            grid[x as usize][y as usize][z as usize] = false;
        }
        for (x, y, z) in new_block {
            grid[x as usize][y as usize][z as usize] = true;
        }
        blocks[i] = new_block;
    }

    let mut supporting = vec![HashSet::new(); blocks.len()];
    let mut supported_by = vec![HashSet::new(); blocks.len()];
    for (i, block) in blocks.clone().into_iter().enumerate() {
        let mut new_block = block;
        new_block.z += 1;
        for (j, other_block) in blocks.clone().into_iter().enumerate().skip(i + 1) {
            for (x, y, z) in new_block {
                if other_block
                    .into_iter()
                    .any(|(x2, y2, z2)| x2 == x && y2 == y && z2 == z)
                    && other_block.into_iter().count() > 0
                {
                    supporting[i].insert(j);
                    supported_by[j].insert(i);
                }
            }
        }
    }

    let count = blocks
        .iter()
        .enumerate()
        .filter(|(i, _)| supporting[*i].iter().all(|j| supported_by[*j].len() >= 2))
        .count();

    println!("First: {}", count);
}

pub fn second() {
    let mut blocks = read_file()
        .lines()
        .map(|s| s.parse::<Block>().unwrap())
        .sorted_unstable_by_key(|block| block.z)
        .collect_vec();

    let (max_x, max_y, max_z) = blocks.iter().fold((0, 0, 0), |acc, block| {
        let point = block.max_point();
        (
            u64::max(acc.0, point.0),
            u64::max(acc.1, point.1),
            u64::max(acc.2, point.2),
        )
    });

    let mut grid =
        vec![vec![vec![false; max_z as usize + 1]; max_y as usize + 1]; max_x as usize + 1];

    for block in blocks.clone() {
        for (x, y, z) in block {
            grid[x as usize][y as usize][z as usize] = true;
        }
    }

    for (i, block) in blocks.clone().into_iter().enumerate() {
        let mut height = block.z as i64 - 1;
        if block.direction == Direction::Z {
            while height >= 0 && !grid[block.x as usize][block.y as usize][height as usize] {
                height -= 1;
            }
        } else {
            while height >= 0 {
                let mut new_block = block;
                new_block.z = height as u64;
                if new_block
                    .into_iter()
                    .any(|(x, y, z)| grid[x as usize][y as usize][z as usize])
                {
                    break;
                };
                height -= 1;
            }
        }
        height += 1;
        let mut new_block = block;
        new_block.z = height as u64;
        for (x, y, z) in block {
            grid[x as usize][y as usize][z as usize] = false;
        }
        for (x, y, z) in new_block {
            grid[x as usize][y as usize][z as usize] = true;
        }
        blocks[i] = new_block;
    }

    blocks = blocks
        .into_iter()
        .sorted_unstable_by_key(|block| block.z)
        .collect_vec();

    let mut supporting = vec![HashSet::new(); blocks.len()];
    let mut supported_by = vec![HashSet::new(); blocks.len()];
    for (i, block) in blocks.clone().into_iter().enumerate() {
        let mut new_block = block;
        new_block.z += 1;
        for (j, other_block) in blocks.clone().into_iter().enumerate().skip(i + 1) {
            for (x, y, z) in new_block {
                if other_block
                    .into_iter()
                    .any(|(x2, y2, z2)| x2 == x && y2 == y && z2 == z)
                    && other_block.into_iter().count() > 0
                {
                    supporting[i].insert(j);
                    supported_by[j].insert(i);
                }
            }
        }
    }

    let grounded = supported_by
        .iter()
        .map(|supported_by| supported_by.is_empty())
        .collect_vec();

    let count = blocks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut supported_by = supported_by.clone();
            let mut count = 0;
            for block in supporting[i].clone() {
                supported_by[block].remove(&i);
            }
            for j in (i + 1)..blocks.len() {
                if grounded[j] {
                    continue;
                }

                if supported_by[j].is_empty() {
                    count += 1;
                    for block in supporting[j].clone() {
                        supported_by[block].remove(&j);
                    }
                }
            }
            count
        })
        .sum::<u64>();

    println!("Second: {}", count);
}
