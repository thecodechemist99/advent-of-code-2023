// Load modules
mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::fs;

fn main() {
    println!(
        "Day 1: {:?}",
        day_1::day_1(read_input("src/day_1/input.txt"))
    );
    println!(
        "Day 2: {:?}",
        day_2::day_2(read_input("src/day_2/input.txt"))
    );
    println!(
        "Day 3: {:?}",
        day_3::day_3(read_input("src/day_3/input.txt"))
    );
    println!(
        "Day 4: {:?}",
        day_4::day_4(read_input("src/day_4/input.txt"))
    );
}

/// Read inputs from text file
fn read_input(dir: &str) -> String {
    fs::read_to_string(dir).unwrap()
}
