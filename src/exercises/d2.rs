use super::Part;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::{Paper, Rock, Scissors};

// note that this is decrement (mod 3)
const fn pick_to_lose(opponents_pick: Shape) -> Shape {
    match opponents_pick {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}

// note that this is increment (mod 3)
const fn pick_to_win(opponents_pick: Shape) -> Shape {
    match opponents_pick {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

// score component from winning or losing
fn win_score(opponents_pick: Shape, our_pick: Shape) -> i32 {
    if our_pick == opponents_pick {
        return 3; // draw
    } else if our_pick == pick_to_win(opponents_pick) {
        return 6; // win
    }
    return 0; // loss
}

// score that we get for picking a certain shape
fn shape_score(our_pick: Shape) -> i32 {
    match our_pick {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

// total score including the "value" or our pick
fn score(opponents_pick: Shape, our_pick: Shape) -> i32 {
    return shape_score(our_pick) + win_score(opponents_pick, our_pick);
}

pub fn solve(part: Part, input: impl Into<String>) -> i32 {
    let s = input.into();

    let mut sum: i32 = 0;
    for l in s.split('\n') {
        if l.is_empty() {
            continue; //skip empty line which may occur at the end
        }

        let mut moves = l.split(' ');
        let opponents_pick = match moves.next().expect("expected opponent move") {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            invalid_mode => panic!("invalid move {}, expected A, B or C", invalid_mode),
        };

        let our_pick = match part {
            Part::One => match moves.next().expect("expected our move") {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                invalid_mode => panic!("invalid move {}, expected X, Y or Z", invalid_mode),
            },
            Part::Two => match moves.next().expect("expected our move") {
                "X" => pick_to_lose(opponents_pick),
                "Y" => opponents_pick,
                "Z" => pick_to_win(opponents_pick),
                invalid_mode => panic!("invalid move {}, expected X, Y or Z", invalid_mode),
            },
        };

        sum += score(opponents_pick, our_pick);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, TEST_INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(solve(Part::Two, TEST_INPUT), 12);
    }
}
