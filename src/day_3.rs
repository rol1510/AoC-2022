use bitmaps::Bitmap;
use itertools::Itertools;

const BITMAP_SIZE: usize = 52;

#[derive(Debug)]
struct Compartment {
    elements: Bitmap<BITMAP_SIZE>,
}

fn item_to_index(item: char) -> usize {
    let item = item as u8;

    fn ascii(char: char) -> u8 {
        char as u8
    }

    if item >= ascii('a') && item <= ascii('z') {
        return (item - ascii('a')) as usize;
    } else if item >= ascii('A') && item <= ascii('Z') {
        return (item - ascii('A')) as usize + 26;
    } else {
        panic!("Invalid item: {}", item as char);
    }
}

impl Compartment {
    fn from(input: &str) -> Compartment {
        let mut elements = Bitmap::new();
        for item in input.chars() {
            elements.set(item_to_index(item), true);
        }
        Compartment { elements }
    }
}

#[derive(Debug)]
struct Packpack {
    com_a: Compartment,
    com_b: Compartment,
}

impl Packpack {
    fn from(items: &str) -> Packpack {
        let parts = items.split_at(items.len() / 2);

        Packpack {
            com_a: Compartment::from(parts.0),
            com_b: Compartment::from(parts.1),
        }
    }
}

pub fn part_1(input: String) -> i32 {
    input
        .lines()
        .map(|line| Packpack::from(line.trim()))
        .map(|packpack| {
            let mut count: i32 = 0;
            for i in 0..BITMAP_SIZE {
                if packpack.com_a.elements.get(i) && packpack.com_b.elements.get(i) {
                    count += i as i32 + 1;
                }
            }
            count
        })
        .sum()
}

pub fn part_2(input: String) -> i32 {
    input
        .lines()
        .map(|line| Compartment::from(line.trim()))
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut bm = Bitmap::<BITMAP_SIZE>::new();
            bm.invert();
            let res = chunk.fold(bm, |acc, compartment| acc & compartment.elements);
            res.first_index().unwrap() as i32 + 1
        })
        .sum::<i32>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(3, Challenge::Sample1)), 157);
        assert_eq!(part_1(get_input(3, Challenge::Challenge1)), 7691);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(3, Challenge::Sample1)), 70);
        assert_eq!(part_2(get_input(3, Challenge::Challenge1)), 2508);
    }
}
