use itertools::Itertools;

trait MyStr {
    fn split_by_empty_lines(&self) -> Vec<&str>;
}

impl MyStr for str {
    fn split_by_empty_lines(&self) -> Vec<&str> {
        self.split("\r\n\r\n").collect()
    }
}

pub fn part_1(input: String) -> u64 {
    let biggest = input
        .split_by_empty_lines()
        .iter()
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max();

    return biggest.unwrap();
}

pub fn part_2(input: String) -> u64 {
    let biggest = input
        .split_by_empty_lines()
        .iter()
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    return biggest;
}
#[cfg(test)]
mod tests {
    use crate::input::Challenge;

    use super::*;
    use crate::input::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(1, Challenge::Sample1)), 24000);
        assert_eq!(part_1(get_input(1, Challenge::Challenge1)), 72240);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(1, Challenge::Sample1)), 45000);
        assert_eq!(part_2(get_input(1, Challenge::Challenge1)), 210957);
    }
}
