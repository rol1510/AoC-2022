mod day_1;
mod day_2;
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

    println!("--- Day 2 ---");
    println!(
        "Part 1: {}",
        day_2::part_1(input::get_input(2, input::Challenge::Challenge1))
    );
    println!(
        "Part 2: {}",
        day_2::part_2(input::get_input(2, input::Challenge::Challenge1))
    );
}
