use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Challenge {
    Sample1,
    Sample2,
    Challenge1,
    Challenge2,
}

fn challenge_to_extension(challenge: Challenge) -> &'static str {
    match challenge {
        Challenge::Sample1 => "sp1.txt",
        Challenge::Sample2 => "sp2.txt",
        Challenge::Challenge1 => "ch1.txt",
        Challenge::Challenge2 => "ch2.txt",
        _ => panic!("Invalid challenge"),
    }
}

fn make_file_path(day: usize, challenge: Challenge) -> String {
    let mut path = PathBuf::new();
    path.push("input");
    path.push(format!("day_{}.{}", day, challenge_to_extension(challenge)));
    return path.to_str().unwrap().to_string();
}

pub fn get_input(day: usize, challenge: Challenge) -> String {
    let path = make_file_path(day, challenge);
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    return contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_file_path() {
        assert_eq!(
            make_file_path(1, Challenge::Sample1),
            "input\\day_1.sp1.txt".to_string()
        );
        assert_eq!(
            make_file_path(1, Challenge::Sample2),
            "input\\day_1.sp2.txt".to_string()
        );
        assert_eq!(
            make_file_path(1, Challenge::Challenge1),
            "input\\day_1.ch1.txt".to_string()
        );
        assert_eq!(
            make_file_path(1, Challenge::Challenge2),
            "input\\day_1.ch2.txt".to_string()
        );

        assert_eq!(
            make_file_path(24, Challenge::Sample1),
            "input\\day_24.sp1.txt".to_string()
        );
    }
}
