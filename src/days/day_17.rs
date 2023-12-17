use std::collections::BinaryHeap;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_17.txt").expect("Error reading the file")
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(&self, (r, c, z): (usize, usize, usize)) -> Option<(usize, usize, usize)> {
        let (prev_direction, prev_distance) = get_direction_distance(z);

        if *self == prev_direction {
            if prev_distance == 3 {
                None
            } else {
                match self {
                    Direction::Up => {
                        if (0..141).contains(&(r as i32 - 1)) {
                            Some((r - 1, c, get_alt_index(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Down => {
                        if (0..141).contains(&(r as i32 + 1)) {
                            Some((r + 1, c, get_alt_index(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Left => {
                        if (0..141).contains(&(c as i32 - 1)) {
                            Some((r, c - 1, get_alt_index(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Right => {
                        if (0..141).contains(&(c as i32 + 1)) {
                            Some((r, c + 1, get_alt_index(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                }
            }
        } else if *self == Direction::Up && prev_direction == Direction::Down
            || *self == Direction::Down && prev_direction == Direction::Up
            || *self == Direction::Left && prev_direction == Direction::Right
            || *self == Direction::Right && prev_direction == Direction::Left
        {
            None
        } else {
            match self {
                Direction::Up => {
                    if (0..141).contains(&(r as i32 - 1)) {
                        Some((r - 1, c, get_alt_index(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if (0..141).contains(&(r as i32 + 1)) {
                        Some((r + 1, c, get_alt_index(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if (0..141).contains(&(c as i32 - 1)) {
                        Some((r, c - 1, get_alt_index(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if (0..141).contains(&(c as i32 + 1)) {
                        Some((r, c + 1, get_alt_index(*self, 1)))
                    } else {
                        None
                    }
                }
            }
        }
    }

    pub fn step_ultra(&self, (r, c, z): (usize, usize, usize)) -> Option<(usize, usize, usize)> {
        let (prev_direction, prev_distance) = get_direction_distance_ultra(z);

        if *self == prev_direction {
            if prev_distance == 10 {
                None
            } else {
                match self {
                    Direction::Up => {
                        if (0..141).contains(&(r as i32 - 1)) {
                            Some((r - 1, c, get_alt_index_ultra(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Down => {
                        if (0..141).contains(&(r as i32 + 1)) {
                            Some((r + 1, c, get_alt_index_ultra(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Left => {
                        if (0..141).contains(&(c as i32 - 1)) {
                            Some((r, c - 1, get_alt_index_ultra(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                    Direction::Right => {
                        if (0..141).contains(&(c as i32 + 1)) {
                            Some((r, c + 1, get_alt_index_ultra(*self, prev_distance + 1)))
                        } else {
                            None
                        }
                    }
                }
            }
        } else if *self == Direction::Up && prev_direction == Direction::Down
            || *self == Direction::Down && prev_direction == Direction::Up
            || *self == Direction::Left && prev_direction == Direction::Right
            || *self == Direction::Right && prev_direction == Direction::Left
            || (1..4).contains(&prev_distance)
        {
            None
        } else {
            match self {
                Direction::Up => {
                    if (0..141).contains(&(r as i32 - 1)) {
                        Some((r - 1, c, get_alt_index_ultra(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if (0..141).contains(&(r as i32 + 1)) {
                        Some((r + 1, c, get_alt_index_ultra(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if (0..141).contains(&(c as i32 - 1)) {
                        Some((r, c - 1, get_alt_index_ultra(*self, 1)))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if (0..141).contains(&(c as i32 + 1)) {
                        Some((r, c + 1, get_alt_index_ultra(*self, 1)))
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn get_alt_index(direction: Direction, distance: usize) -> usize {
    direction as usize * 4 + distance
}

fn get_direction_distance(alt_index: usize) -> (Direction, usize) {
    let distance = alt_index % 4;
    let direction = alt_index / 4;
    (direction.into(), distance)
}

fn get_alt_index_ultra(direction: Direction, distance: usize) -> usize {
    direction as usize + distance * 4
}

fn get_direction_distance_ultra(alt_index: usize) -> (Direction, usize) {
    let distance = alt_index / 4;
    let direction = alt_index % 4;
    (direction.into(), distance)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: Vec<Vec<u32>>, start: (usize, usize, usize), end: (usize, usize)) -> usize {
    let mut max_distances: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 16]; 141]; 141];

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });
    max_distances[start.0][start.1][start.2] = Some(0);

    while let Some(State { cost, position }) = heap.pop() {
        if position.0 == end.0 && position.1 == end.1 {
            return cost;
        }

        if let Some(max_distance) = max_distances[position.0][position.1][position.2] {
            if cost > max_distance {
                continue;
            }
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|direction| direction.step(position))
        .filter_map(|new_position| {
            let next = State {
                cost: cost + grid[new_position.0][new_position.1] as usize,
                position: new_position,
            };

            if next.cost
                < max_distances[new_position.0][new_position.1][new_position.2]
                    .unwrap_or(usize::MAX)
            {
                max_distances[new_position.0][new_position.1][new_position.2] = Some(next.cost);
                Some(next)
            } else {
                None
            }
        })
        .for_each(|next| {
            heap.push(next);
        });
    }

    0
}

fn dijkstra_ultra(grid: Vec<Vec<u32>>, start: (usize, usize, usize), end: (usize, usize)) -> usize {
    let mut max_distances: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 44]; 141]; 141];

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });
    max_distances[start.0][start.1][start.2] = Some(0);

    while let Some(State { cost, position }) = heap.pop() {
        if position.0 == end.0 && position.1 == end.1 {
            return cost;
        }

        if let Some(max_distance) = max_distances[position.0][position.1][position.2] {
            if cost > max_distance {
                continue;
            }
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|direction| direction.step_ultra(position))
        .filter_map(|new_position| {
            let next = State {
                cost: cost + grid[new_position.0][new_position.1] as usize,
                position: new_position,
            };

            if next.cost
                < max_distances[new_position.0][new_position.1][new_position.2]
                    .unwrap_or(usize::MAX)
            {
                max_distances[new_position.0][new_position.1][new_position.2] = Some(next.cost);
                Some(next)
            } else {
                None
            }
        })
        .for_each(|next| {
            heap.push(next);
        });
    }

    0
}

pub fn first() {
    let grid = read_file()
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = dijkstra(grid, (0, 0, 0), (140, 140));

    println!("Result: {}", result);
}

pub fn second() {
    let grid = read_file()
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = dijkstra_ultra(grid, (0, 0, 0), (140, 140));

    println!("Result: {}", result);
}
