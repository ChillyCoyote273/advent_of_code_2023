use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_25.txt").expect("Error reading the file")
}

fn betweenness_centrality(graph: &[Vec<usize>]) -> Vec<f64> {
    let mut betweennesses = vec![0.0; graph.len()];
    for s in 0..graph.len() {
        let mut distances = Vec::new();
        let mut p = vec![Vec::new(); graph.len()];
        let mut sigma = vec![0; graph.len()];
        sigma[s] = 1;
        let mut d = vec![-1; graph.len()];
        d[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            distances.push(v);
            for &w in graph[v].iter() {
                if d[w] < 0 {
                    queue.push_back(w);
                    d[w] = d[v] + 1;
                }

                if d[w] == d[v] + 1 {
                    sigma[w] += sigma[v];
                    p[w].push(v);
                }
            }
        }

        let mut delta = vec![0.0; graph.len()];
        while let Some(w) = distances.pop() {
            for &v in p[w].iter() {
                delta[v] += sigma[v] as f64 / sigma[w] as f64 * (1.0 + delta[w]);
            }
            if w != s {
                betweennesses[w] += delta[w];
            }
        }
    }

    betweennesses
}

pub fn first() {
    let graph_structure: HashMap<String, Vec<String>> = read_file()
        .lines()
        .map(|line| {
            let name = line[0..3].to_string();
            let connections = line[5..].split(' ').map(|s| s.to_string()).collect_vec();
            (name, connections)
        })
        .collect();

    let components = graph_structure
        .iter()
        .flat_map(|(k, v)| {
            let mut all = v.clone();
            all.push(k.clone());
            all
        })
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let mut graph = vec![Vec::new(); components.len()];
    for (k, v) in graph_structure {
        let (key, _) = components.iter().find_position(|&x| *x == k).unwrap();
        for destination in v {
            let (value, _) = components
                .iter()
                .find_position(|&x| *x == destination)
                .unwrap();
            graph[key].push(value);
            graph[value].push(key);
        }
    }

    let mut graph = graph
        .iter()
        .map(|node| node.iter().copied().sorted_unstable().dedup().collect_vec())
        .collect_vec();

    let betweenness_centralities = betweenness_centrality(&graph);
    let most_important = betweenness_centralities
        .iter()
        .copied()
        .enumerate()
        .sorted_unstable_by_key(|&(_, v)| (v * 1e6) as i64)
        .rev()
        .collect_vec();

    let edges = (0..3)
        .map(|i| (most_important[2 * i].0, most_important[2 * i + 1].0))
        .collect_vec();

    for edge in edges {
        graph[edge.0] = graph[edge.0]
            .iter()
            .copied()
            .filter(|&e| e != edge.1)
            .collect_vec();
        graph[edge.1] = graph[edge.1]
            .iter()
            .copied()
            .filter(|&e| e != edge.0)
            .collect_vec();
    }

    let mut reachable = vec![false; graph.len()];
    let mut stack = Vec::new();
    stack.push(0);
    reachable[0] = true;
    while let Some(v) = stack.pop() {
        for &w in graph[v].iter() {
            if !reachable[w] {
                reachable[w] = true;
                stack.push(w);
            }
        }
    }
    let a = reachable.iter().filter(|&&x| x).count();
    let b = graph.len() - a;

    println!("{}", a * b);
}
