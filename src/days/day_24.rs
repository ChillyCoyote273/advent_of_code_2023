use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_24.txt").expect("Error reading the file")
}

pub fn first() {
    let rx = regex::Regex::new(r"(?m)^(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)$").unwrap();
    let stones: Vec<((f64, f64), (f64, f64))> = rx
        .captures_iter(&read_file())
        .map(|cap| {
            let (_, [x, y, _, dx, dy, _]) = cap.extract();
            (
                (x.parse().unwrap(), y.parse().unwrap()),
                (dx.parse().unwrap(), dy.parse().unwrap()),
            )
        })
        .collect();

    let mut intersection_count = 0;
    for i in 1..stones.len() {
        for j in 0..i {
            let ((x1, y1), (dx1, dy1)) = stones[i];
            let ((x2, y2), (dx2, dy2)) = stones[j];
            let m1 = dy1 / dx1;
            let m2 = dy2 / dx2;
            let b1 = y1 - m1 * x1;
            let b2 = y2 - m2 * x2;
            if m1 == m2 {
                // parallel
                if b1 != b2 {
                    // different lines
                    continue;
                }

                // same line
                if dx1 == dx2 {
                    // same speed
                    continue;
                }

                // different speed
                let t = (x2 - x1) / (dx1 - dx2);
                if t < 0.0 {
                    // met in past
                    continue;
                }

                // met in future
                let x = x1 + dx1 * t;
                let y = y1 + dy1 * t;
                if 200000000000000.0 < x
                    && x < 400000000000000.0
                    && 200000000000000.0 < y
                    && y < 400000000000000.0
                {
                    intersection_count += 1;
                }
                continue;
            }

            // not parallel
            let x = (b2 - b1) / (m1 - m2);
            let y = m1 * x + b1;

            let t1 = (x - x1) / dx1;
            let t2 = (x - x2) / dx2;
            if t1 < 0.0 || t2 < 0.0 {
                // met in past
                continue;
            }

            // met in future inside box
            if 200000000000000.0 < x
                && x < 400000000000000.0
                && 200000000000000.0 < y
                && y < 400000000000000.0
            {
                intersection_count += 1;
            }
        }
    }

    println!("{}", intersection_count);
}

fn transform_stone(
    frame: ((i128, i128, i128), (i128, i128, i128)),
    stone: ((i128, i128, i128), (i128, i128, i128)),
) -> ((i128, i128, i128), (i128, i128, i128)) {
    (
        (
            stone.0 .0 - frame.0 .0,
            stone.0 .1 - frame.0 .1,
            stone.0 .2 - frame.0 .2,
        ),
        (
            stone.1 .0 - frame.1 .0,
            stone.1 .1 - frame.1 .1,
            stone.1 .2 - frame.1 .2,
        ),
    )
}

fn add(first: (i128, i128, i128), second: (i128, i128, i128)) -> (i128, i128, i128) {
    (first.0 + second.0, first.1 + second.1, first.2 + second.2)
}

fn subtract(first: (i128, i128, i128), second: (i128, i128, i128)) -> (i128, i128, i128) {
    (first.0 - second.0, first.1 - second.1, first.2 - second.2)
}

fn dot(first: (i128, i128, i128), second: (i128, i128, i128)) -> i128 {
    first.0 * second.0 + first.1 * second.1 + first.2 * second.2
}

fn mult(first: (i128, i128, i128), lambda: i128) -> (i128, i128, i128) {
    (first.0 * lambda, first.1 * lambda, first.2 * lambda)
}

fn div(first: (i128, i128, i128), lambda: i128) -> (i128, i128, i128) {
    assert_eq!(
        (first.0 % lambda, first.1 % lambda, first.2 % lambda),
        (0, 0, 0)
    );
    (first.0 / lambda, first.1 / lambda, first.2 / lambda)
}

pub fn second() {
    let rx = regex::Regex::new(r"(?m)^(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)$").unwrap();
    let stones: Vec<((i128, i128, i128), (i128, i128, i128))> = rx
        .captures_iter(&read_file()) //"19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @  1, -5, -3")
        .map(|cap| {
            let (_, [x, y, z, dx, dy, dz]) = cap.extract();
            (
                (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
                (
                    dx.parse().unwrap(),
                    dy.parse().unwrap(),
                    dz.parse().unwrap(),
                ),
            )
        })
        .collect();

    let frame = stones[0];
    let origin = (0, 0, 0);
    let line = transform_stone(frame, stones[1]);
    let second_point = (line.0 .0, line.0 .1, line.0 .2);
    let third_point = add(line.0, line.1);
    let vector_a = subtract(second_point, origin); // no-op
    let vector_b = subtract(third_point, origin); // no-op
    let normal = (
        vector_a.1 * vector_b.2 - vector_a.2 * vector_b.1,
        vector_a.2 * vector_b.0 - vector_a.0 * vector_b.2,
        vector_a.0 * vector_b.1 - vector_a.1 * vector_b.0,
    );

    let stone_a = transform_stone(frame, stones[2]);
    let time_num_a = dot(subtract(origin, stone_a.0), normal);
    let time_den_a = dot(stone_a.1, normal);
    let time_a = time_num_a / time_den_a;
    assert_eq!(time_num_a % time_den_a, 0);
    let intersection_a = add(stone_a.0, mult(stone_a.1, time_a));

    let stone_b = transform_stone(frame, stones[3]);
    let time_num_b = dot(subtract(origin, stone_b.0), normal);
    let time_den_b = dot(stone_b.1, normal);
    let time_b = time_num_b / time_den_b;
    assert_eq!(time_num_b % time_den_b, 0);
    let intersection_b = add(stone_b.0, mult(stone_b.1, time_b));

    let intersection_diff = subtract(intersection_a, intersection_b);
    let time_diff = time_a - time_b;
    let velocity_frame = div(intersection_diff, time_diff);
    let position_frame = subtract(intersection_a, mult(velocity_frame, time_a));

    let position_true = add(frame.0, position_frame);

    println!("{}", dot(position_true, (1, 1, 1)));
}
