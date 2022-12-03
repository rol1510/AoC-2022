mod day_1;
mod input;

fn main() {
    println!(" --- Day 1 --- ");
    println!(
        "Part 1: {}",
        day_1::part1(input::get_input(1, input::Challenge::Sample1))
    );
}
