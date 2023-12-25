mod days;

use days::*;

fn main() {
    let start = std::time::Instant::now();

    print!("Day 1:\nFirst: ");
    day_1::first();
    print!("Second: ");
    day_1::second();

    print!("\nDay 2:\nFirst: ");
    day_2::first();
    print!("Second: ");
    day_2::second();

    print!("\nDay 3:\nFirst: ");
    day_3::first();
    print!("Second: ");
    day_3::second();

    print!("\nDay 4:\nFirst: ");
    day_4::first();
    print!("Second: ");
    day_4::second();

    print!("\nDay 5:\nFirst: ");
    day_5::first();
    print!("Second: ");
    day_5::second();

    print!("\nDay 6:\nFirst: ");
    day_6::first();
    print!("Second: ");
    day_6::second();

    print!("\nDay 7:\nFirst: ");
    day_7::first();
    print!("Second: ");
    day_7::second();

    print!("\nDay 8:\nFirst: ");
    day_8::first();
    print!("Second: ");
    day_8::second();

    print!("\nDay 9:\nFirst: ");
    day_9::first();
    print!("Second: ");
    day_9::second();

    print!("\nDay 10:\nFirst: ");
    day_10::first();
    print!("Second: ");
    day_10::second();

    print!("\nDay 11:\nFirst: ");
    day_11::first();
    print!("Second: ");
    day_11::second();

    print!("\nDay 12:\nFirst: ");
    day_12::first();
    print!("Second: ");
    day_12::second();

    print!("\nDay 13:\nFirst: ");
    day_13::first();
    print!("Second: ");
    day_13::second();

    print!("\nDay 14:\nFirst: ");
    day_14::first();
    print!("Second: ");
    day_14::second();

    print!("\nDay 15:\nFirst: ");
    day_15::first();
    print!("Second: ");
    day_15::second();

    print!("\nDay 16:\nFirst: ");
    day_16::first();
    print!("Second: ");
    day_16::second();

    print!("\nDay 17:\nFirst: ");
    day_17::first();
    print!("Second: ");
    day_17::second();

    print!("\nDay 18:\nFirst: ");
    day_18::first();
    print!("Second: ");
    day_18::second();

    print!("\nDay 19:\nFirst: ");
    day_19::first();
    print!("Second: ");
    day_19::second();

    print!("\nDay 20:\nFirst: ");
    day_20::first();
    print!("Second: ");
    day_20::second();

    print!("\nDay 21:\nFirst: ");
    day_21::first();
    print!("Second: ");
    day_21::second();

    print!("\nDay 22:\nFirst: ");
    day_22::first();
    print!("Second: ");
    day_22::second();

    print!("\nDay 23:\nFirst: ");
    day_23::first();
    print!("Second: ");
    day_23::second();

    print!("\nDay 24:\nFirst: ");
    day_24::first();
    print!("Second: ");
    day_24::second();

    print!("\nDay 25:\nFirst: ");
    day_25::first();

    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;

    println!("\n\nTotal time: {}m {}s", minutes, seconds);
}
