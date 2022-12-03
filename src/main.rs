mod day_1;
mod input;

fn main() {
    println!("--- Day 1 ---");
    println!(
        "Part 1: {}",
        day_1::part_1(input::get_input(1, input::Challenge::Challenge1))
    );
    println!(
        "Part 2: {}",
        day_1::part_2(input::get_input(1, input::Challenge::Challenge1))
    );
}
