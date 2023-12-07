use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_7.txt").expect("Error reading the file")
}

fn five_of_a_kind(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    hand[0] == hand[4]
}

fn four_of_a_kind(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    (hand[0] == hand[3]) || (hand[1] == hand[4])
}

fn full_house(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    (hand[0] == hand[2]) && (hand[3] == hand[4]) || (hand[0] == hand[1]) && (hand[2] == hand[4])
}

fn three_of_a_kind(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    (hand[0] == hand[2]) || (hand[1] == hand[3]) || (hand[2] == hand[4])
}

fn two_pairs(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    (hand[0] == hand[1]) && (hand[2] == hand[3])
        || (hand[0] == hand[1]) && (hand[3] == hand[4])
        || (hand[1] == hand[2]) && (hand[3] == hand[4])
}

fn one_pair(hand: Vec<u64>) -> bool {
    let mut hand = hand;
    hand.sort();
    (hand[0] == hand[1]) || (hand[1] == hand[2]) || (hand[2] == hand[3]) || (hand[3] == hand[4])
}

pub fn first() {
    let mut hands: Vec<(u64, u64)> = read_file()
        .split_terminator('\n')
        .map(|line| {
            let line_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let hand: Vec<u64> = line_parts[0]
                .chars()
                .map(|c| match c {
                    'A' => 14u64,
                    'K' => 13u64,
                    'Q' => 12u64,
                    'J' => 11u64,
                    'T' => 10u64,
                    _ => c.to_digit(10).unwrap() as u64,
                })
                .collect();
            let bid = line_parts[1].parse::<u64>().unwrap();
            let hand_rank = if five_of_a_kind(hand.clone()) {
                6
            } else if four_of_a_kind(hand.clone()) {
                5
            } else if full_house(hand.clone()) {
                4
            } else if three_of_a_kind(hand.clone()) {
                3
            } else if two_pairs(hand.clone()) {
                2
            } else if one_pair(hand.clone()) {
                1
            } else {
                0
            };
            let mut rating = hand_rank;
            for card in hand {
                rating = rating * 100 + card;
            }
            (rating, bid)
        })
        .collect();
    hands.sort_by_key(|hand| hand.0);
    let mut acc = 0u64;
    for (i, hand) in hands.iter().enumerate() {
        acc += hand.1 * (i as u64 + 1);
    }
    println!("{}", acc);
}

pub fn second() {
    let mut hands: Vec<(u64, u64)> = read_file()
        .split_terminator('\n')
        .map(|line| {
            let line_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let hand: Vec<u64> = line_parts[0]
                .chars()
                .map(|c| match c {
                    'A' => 13u64,
                    'K' => 12u64,
                    'Q' => 11u64,
                    'J' => 1u64,
                    'T' => 10u64,
                    _ => c.to_digit(10).unwrap() as u64,
                })
                .collect();
            let bid = line_parts[1].parse::<u64>().unwrap();
            let hand_rank = (2..=13)
                .map(|sub| {
                    let mut hand = hand.clone();
                    for card in hand.iter_mut() {
                        if *card == 1 {
                            *card = sub;
                        }
                    }
                    if five_of_a_kind(hand.clone()) {
                        6
                    } else if four_of_a_kind(hand.clone()) {
                        5
                    } else if full_house(hand.clone()) {
                        4
                    } else if three_of_a_kind(hand.clone()) {
                        3
                    } else if two_pairs(hand.clone()) {
                        2
                    } else if one_pair(hand.clone()) {
                        1
                    } else {
                        0
                    }
                })
                .max()
                .unwrap();
            let mut rating = hand_rank;
            for card in hand {
                rating = rating * 100 + card;
            }
            (rating, bid)
        })
        .collect();
    hands.sort_by_key(|hand| hand.0);
    let mut acc = 0u64;
    for (i, hand) in hands.iter().enumerate() {
        acc += hand.1 * (i as u64 + 1);
    }
    println!("{}", acc);
}
