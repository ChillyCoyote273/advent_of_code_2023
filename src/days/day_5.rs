use itertools::Itertools;
use std::fs;
use std::ops::Range;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_5.txt").expect("Error reading the file")
}

fn map_item(map: &Vec<(Range<u64>, Range<u64>)>, number: u64) -> u64 {
    for (destination, source) in map {
        if source.contains(&number) {
            return destination.start + (number - source.start);
        }
    }
    number
}

fn map_ranges(map: &[(Range<u64>, Range<u64>)], ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let breaks: Vec<u64> = map
        .iter()
        .flat_map(|(_, source)| vec![source.start, source.end])
        .unique()
        .collect();

    let broken_ranges: Vec<Range<u64>> = ranges
        .iter()
        .flat_map(|range| {
            let mut break_points = vec![range.start, range.end];
            for break_point in &breaks {
                if range.contains(break_point) {
                    break_points.push(*break_point);
                }
            }
            break_points.sort();
            break_points
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(&start, &end)| start..end)
                .collect::<Vec<Range<u64>>>()
        })
        .map(|range| {
            map.iter()
                .find(|(_, source)| source.contains(&range.start))
                .map_or(range.clone(), |(destination, source)| {
                    (range.start + destination.start - source.start)
                        ..(range.end + destination.end - source.end)
                })
        })
        .collect();

    let mut combined_ranges = vec![];
    let mut current_range = 0;
    while current_range < broken_ranges.len() {
        let mut range = broken_ranges[current_range].clone();
        while current_range < broken_ranges.len() - 1
            && range.end == broken_ranges[current_range + 1].start
        {
            range = range.start..broken_ranges[current_range + 1].end;
            current_range += 1;
        }
        combined_ranges.push(range);
        current_range += 1;
    }

    combined_ranges
}

pub fn first() {
    let input = read_file();
    let seeds: Vec<u64> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let maps: Vec<Vec<(Range<u64>, Range<u64>)>> = input
        .split("\n\n")
        .skip(1)
        .map(|single_map| {
            single_map
                .split('\n')
                .skip(1)
                .map(|mapping| {
                    let mapping_ranges = mapping
                        .split(' ')
                        .map(|elem| elem.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (
                        mapping_ranges[0]..(mapping_ranges[0] + mapping_ranges[2]),
                        mapping_ranges[1]..(mapping_ranges[1] + mapping_ranges[2]),
                    )
                })
                .collect()
        })
        .collect();
    let output = seeds
        .iter()
        .map(|seed| {
            let mut item = *seed;
            for map in &maps {
                item = map_item(map, item);
            }
            item
        })
        .min()
        .unwrap();

    println!("{}", output);
}

pub fn second() {
    let input = read_file();
    let seeds: Vec<Vec<Range<u64>>> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let pair = chunk.collect::<Vec<&str>>();
            let start: u64 = pair[0].parse().unwrap();
            let length: u64 = pair[1].parse().unwrap();
            vec![start..(start + length)]
        })
        .collect();

    let maps: Vec<Vec<(Range<u64>, Range<u64>)>> = input
        .split("\n\n")
        .skip(1)
        .map(|single_map| {
            single_map
                .split('\n')
                .skip(1)
                .map(|mapping| {
                    let mapping_ranges = mapping
                        .split(' ')
                        .map(|elem| elem.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (
                        mapping_ranges[0]..(mapping_ranges[0] + mapping_ranges[2]),
                        mapping_ranges[1]..(mapping_ranges[1] + mapping_ranges[2]),
                    )
                })
                .collect()
        })
        .collect();

    let output = seeds
        .iter()
        .map(|seed| {
            let mut item = seed.clone();
            for map in &maps {
                item = map_ranges(map, item);
            }
            item.iter().map(|range| range.start).min().unwrap()
        })
        .min()
        .unwrap();

    println!("{}", output);
}
