use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_23.txt").expect("Error reading the file")
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn find_nodes(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut nodes = Vec::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '#' {
                continue;
            }

            if [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|&(r_offset, c_offset)| {
                    if (0..map.len() as i32).contains(&(r as i32 + r_offset))
                        && (0..map[r].len() as i32).contains(&(c as i32 + c_offset))
                    {
                        if map[(r as i32 + r_offset) as usize][(c as i32 + c_offset) as usize]
                            != '#'
                        {
                            1
                        } else {
                            0
                        }
                    } else {
                        2
                    }
                })
                .sum::<i32>()
                > 2
            {
                nodes.push((r, c));
            }
        }
    }
    nodes.into_iter().sorted_unstable().collect_vec()
}

fn find_distances(map: &[Vec<char>], nodes: &[(usize, usize)]) -> Vec<(usize, usize, usize)> {
    let mut edges = Vec::new();

    for (i, &(r, c)) in nodes.iter().enumerate() {
        for mut direction in [(0, 1), (1, 0)] {
            let mut current_position = (r as i32 + direction.0, c as i32 + direction.1);
            if !(0..map.len() as i32).contains(&current_position.0)
                || !(0..map[0].len() as i32).contains(&current_position.1)
            {
                continue;
            }
            if map[current_position.0 as usize][current_position.1 as usize] == '#' {
                continue;
            }
            let mut distance = 1;
            while !nodes.contains(&(current_position.0 as usize, current_position.1 as usize)) {
                let symbol = map[current_position.0 as usize][current_position.1 as usize];
                if symbol == '>' {
                    direction = (0, 1);
                    current_position.1 += 1;
                    distance += 1;
                    continue;
                }
                if symbol == 'v' {
                    direction = (1, 0);
                    current_position.0 += 1;
                    distance += 1;
                    continue;
                }
                for next_direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if next_direction == (-direction.0, -direction.1) {
                        continue;
                    }
                    let next_position = (
                        current_position.0 + next_direction.0,
                        current_position.1 + next_direction.1,
                    );
                    if map[next_position.0 as usize][next_position.1 as usize] != '#' {
                        current_position = next_position;
                        direction = next_direction;
                        distance += 1;
                        break;
                    }
                }
            }

            let (j, _) = nodes
                .iter()
                .find_position(|&&n| (n.0 as i32, n.1 as i32) == current_position)
                .unwrap();

            edges.push((i, j, distance));
        }
    }

    edges
}

fn get_adjacency(edges: &[(usize, usize, usize)], size: usize) -> Vec<Vec<Edge>> {
    let mut adjacency = vec![Vec::new(); size];
    for &(from, to, distance) in edges {
        adjacency[from].push(Edge {
            node: to,
            cost: distance,
        });
    }

    for node in adjacency.iter_mut() {
        node.sort_unstable_by_key(|edge| edge.node);
    }

    adjacency
}

fn delete_node(adjacency: &mut [Vec<Edge>], node: usize) {
    for edges in adjacency.iter_mut() {
        for i in 0..edges.len() {
            if edges[i].node == node {
                edges.remove(i);
                break;
            }
        }
    }
    adjacency[node].clear();
}

fn get_longest_path(adjacency: &[Vec<Edge>], current: usize, goal: usize) -> Option<usize> {
    if current == goal {
        return Some(0);
    }
    shortest_path(adjacency, current, goal)?;

    let adjacent_nodes = &adjacency[current];
    let mut adjacency = adjacency.to_vec();
    delete_node(&mut adjacency, current);

    let mut max_distance: Option<usize> = None;
    for &Edge {
        node: next,
        cost: distance,
    } in adjacent_nodes
    {
        if let Some(other_distance) = get_longest_path(&adjacency, next, goal) {
            if let Some(max_distance) = max_distance.as_mut() {
                *max_distance = usize::max(*max_distance, distance + other_distance);
            } else {
                max_distance = Some(distance + other_distance);
            }
        }
    }

    max_distance
}

pub fn first() {
    let map = read_file()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let nodes = find_nodes(&map);

    let edges = find_distances(&map, &nodes);

    let adjacency = get_adjacency(&edges, nodes.len());

    let longest_path = get_longest_path(&adjacency, 0, nodes.len() - 1).unwrap();

    println!("{}", longest_path);
}

pub fn second() {
    let map = read_file()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let nodes = find_nodes(&map);

    let edges = find_distances(&map, &nodes);
    let edges = edges
        .into_iter()
        .flat_map(|(from, to, distance)| [(from, to, distance), (to, from, distance)])
        .collect_vec();

    let adjacency = get_adjacency(&edges, nodes.len());

    let longest_path = get_longest_path(&adjacency, 0, nodes.len() - 1).unwrap();

    println!("{}", longest_path);
}
