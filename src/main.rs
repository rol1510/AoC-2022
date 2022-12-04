use easy_min_max::max;
use std::ops::Add;

mod day_1;
mod day_2;
mod day_3;
mod input;

const BOX_WIDTH: i32 = 20;

fn box_line(s: String, width: i32) -> String {
    let padding_needed = max!(0, width - 4 - s.len() as i32);

    let padding = string_of_n_chars(' ', padding_needed);

    return format!("| {}{} |", s, padding);
}

fn get_header(day: i32, width: i32) -> String {
    let day_string = format!(" Day {} ", day);
    return get_separator(day_string, width);
}

fn get_separator(msg: String, width: i32) -> String {
    let padding_needed = max!(0, width - 2 - msg.len() as i32);

    let padding_left = padding_needed / 2;
    let padding_right = padding_needed - padding_left;

    let padding_left = string_of_n_chars('-', padding_left);
    let padding_right = string_of_n_chars('-', padding_right);

    return format!("+{}{}{}+", padding_left, msg, padding_right);
}

fn string_of_n_chars(c: char, n: i32) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s = s.add(&c.to_string());
    }
    return s;
}

macro_rules! output_day {
    ($number:expr, $day:tt) => {
        println!("{}", get_header($number, BOX_WIDTH));
        let p1 = format!(
            "Part 1: {}",
            $day::part_1(input::get_input($number, input::Challenge::Challenge1))
        );
        println!("{}", box_line(p1, BOX_WIDTH));

        let p1 = format!(
            "Part 2: {}",
            $day::part_2(input::get_input($number, input::Challenge::Challenge1))
        );
        println!("{}", box_line(p1, BOX_WIDTH));
    };
}

fn main() {
    output_day!(1, day_1);
    output_day!(2, day_2);
    output_day!(3, day_3);

    println!("{}", get_separator("".to_string(), BOX_WIDTH));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_box_line() {
        assert_eq!(box_line("test".to_string(), 10), "| test   |");
        assert_eq!(box_line("test".to_string(), 8), "| test |");
        assert_eq!(box_line("test".to_string(), 4), "| test |");
    }

    #[test]
    fn test_get_separator() {
        assert_eq!(get_separator("test".to_string(), 10), "+--test--+");
        assert_eq!(get_separator("test".to_string(), 8), "+-test-+");
        assert_eq!(get_separator("test".to_string(), 6), "+test+");
        assert_eq!(get_separator("test".to_string(), 4), "+test+");

        assert_eq!(get_separator("test".to_string(), 11), "+--test---+");
        assert_eq!(get_separator("test".to_string(), 7), "+test-+");
    }
}
