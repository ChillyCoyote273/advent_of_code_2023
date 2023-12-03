use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_2.txt").expect("Error reading the file")
}

pub fn first() {
    let colors = ["red", "green", "blue"];
    let maximums = [12, 13, 14];
    let result: usize = read_file()
        .split_terminator("\n")
        .map(|game| {
            game.chars()
                .skip_while(|&c| c != ':')
                .skip(2)
                .collect::<String>()
                .split("; ")
                .map(|hand| {
                    hand.split(", ")
                        .map(|color| {
                            let color_data: Vec<_> = color.split(" ").collect();
                            let mut color_array = [0; 3];
                            color_array[colors.iter().position(|&c| c == color_data[1]).unwrap()] =
                                color_data[0].parse::<i32>().unwrap();
                            color_array.clone()
                        })
                        .fold(vec![0; 3], |acc, x| {
                            acc.iter().zip(x.iter()).map(|(&a, &b)| a + b).collect()
                        })
                })
                .fold(vec![0; 3], |acc, x| {
                    acc.iter().zip(x.iter()).map(|(&a, &b)| a.max(b)).collect()
                })
        })
        .enumerate()
        .filter_map(|(i, game)| {
            if game.iter().zip(maximums.iter()).all(|(&a, &b)| a <= b) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    println!("{:?}", result);
}

pub fn second() {
    let colors = ["red", "green", "blue"];
    let result: i32 = read_file()
        .split_terminator("\n")
        .map(|game| {
            game.chars()
                .skip_while(|&c| c != ':')
                .skip(2)
                .collect::<String>()
                .split("; ")
                .map(|hand| {
                    hand.split(", ")
                        .map(|color| {
                            let color_data: Vec<_> = color.split(" ").collect();
                            let mut color_array = [0; 3];
                            color_array[colors.iter().position(|&c| c == color_data[1]).unwrap()] =
                                color_data[0].parse::<i32>().unwrap();
                            color_array.clone()
                        })
                        .fold(vec![0; 3], |acc, x| {
                            acc.iter().zip(x.iter()).map(|(&a, &b)| a + b).collect()
                        })
                })
                .fold(vec![0; 3], |acc, x| {
                    acc.iter().zip(x.iter()).map(|(&a, &b)| a.max(b)).collect()
                })
                .iter()
                .product::<i32>()
        })
        .sum();
    println!("{:?}", result);
}
