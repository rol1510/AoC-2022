#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn parse_line_as_shapes(line: String) -> (Shape, Shape) {
    let mut shapes = line.split_whitespace();
    let shape1 = match shapes.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _ => panic!("Invalid shape"),
    };
    let shape2 = match shapes.next().unwrap() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissor,
        _ => panic!("Invalid shape"),
    };
    return (shape1, shape2);
}

fn parse_line_as_shape_and_result(line: String) -> (Shape, Outcome) {
    let mut shapes = line.split_whitespace();
    let shape1 = match shapes.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _ => panic!("Invalid shape"),
    };
    let shape2 = match shapes.next().unwrap() {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid shape"),
    };
    return (shape1, shape2);
}

fn game_result(moves: &(Shape, Shape)) -> i32 {
    match moves.0 {
        Shape::Rock => match moves.1 {
            Shape::Rock => 0,
            Shape::Paper => -1,
            Shape::Scissor => 1,
        },
        Shape::Paper => match moves.1 {
            Shape::Rock => 1,
            Shape::Paper => 0,
            Shape::Scissor => -1,
        },
        Shape::Scissor => match moves.1 {
            Shape::Rock => -1,
            Shape::Paper => 1,
            Shape::Scissor => 0,
        },
    }
}

fn score_for_outcome(moves: &(Shape, Shape)) -> i32 {
    match game_result(moves) {
        1 => 0,
        0 => 3,
        -1 => 6,
        _ => panic!("Invalid game result"),
    }
}

fn score_for_shape(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn calculate_score(moves: (Shape, Shape)) -> i32 {
    let outcome = score_for_outcome(&moves);
    let shape = score_for_shape(&moves.1);
    return outcome + shape;
}

fn get_shape_for_outcome(opponents_shape: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Win => match opponents_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        },
        Outcome::Lose => match opponents_shape {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        },
        Outcome::Draw => opponents_shape,
    }
}

pub fn part_1(input: String) -> i32 {
    input
        .lines()
        .map(|line| parse_line_as_shapes(line.to_string()))
        .map(|moves| calculate_score(moves))
        .sum()
}

pub fn part_2(input: String) -> i32 {
    input
        .lines()
        .map(|line| parse_line_as_shape_and_result(line.to_string()))
        .map(|(opponents_shape, outcome)| {
            let my_shape = get_shape_for_outcome(opponents_shape, outcome);
            calculate_score((opponents_shape, my_shape))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(2, Challenge::Sample1)), 15);
        assert_eq!(part_1(get_input(2, Challenge::Challenge1)), 12645);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(2, Challenge::Sample1)), 12);
        assert_eq!(part_2(get_input(2, Challenge::Challenge1)), 11756);
    }

    #[test]
    fn test_score() {
        assert_eq!(calculate_score(parse_line_as_shapes("A X".to_string())), 4);
        assert_eq!(calculate_score(parse_line_as_shapes("B Y".to_string())), 5);
        assert_eq!(calculate_score(parse_line_as_shapes("C Z".to_string())), 6);

        assert_eq!(calculate_score(parse_line_as_shapes("A Y".to_string())), 8);
        assert_eq!(calculate_score(parse_line_as_shapes("A Z".to_string())), 3);
    }
}
