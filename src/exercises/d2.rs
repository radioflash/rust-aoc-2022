use super::Part;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::{Paper, Rock, Scissors};

// note that this is decrement (mod 3)
const fn pick_to_lose(opp: Shape) -> Shape {
    match opp {
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
fn winscore(opponents_pick: Shape, our_pick: Shape) -> i32 {
    if our_pick == pick_to_win(opponents_pick) {
        return 6;
    } else if our_pick == pick_to_lose(opponents_pick) {
        return 0;
    }
    return 3;
}

// total score including the "value" or our pick
fn total_score(opponents_pick: Shape, our_pick: Shape) -> i32 {
    match (opponents_pick, our_pick) {
        (opp, Rock) => 1 + winscore(opp, Rock),
        (opp, Paper) => 2 + winscore(opp, Paper),
        (opp, Scissors) => 3 + winscore(opp, Scissors),
    }
}

pub fn solve(part: Part, input: impl Into<String>) -> i32 {
    let s = input.into();
    let mut sum: i32 = 0;
    for l in s.split('\n') {
        if l.is_empty() {
            break;
        }

        let mut moves = l.split(' ');
        let opponents_pick = match moves.nth(0).expect("expected opponent move") {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            invalid_mode => panic!("invalid move {}, expected A, B or C", invalid_mode),
        };

        let our_pick = match part {
            Part::One => match moves.nth(0).expect("expected our move") {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                invalid_mode => panic!("invalid move {}, expected X, Y or Z", invalid_mode),
            },
            Part::Two => match moves.nth(0).expect("expected our move") {
                "X" => pick_to_lose(opponents_pick),
                "Y" => opponents_pick,
                "Z" => pick_to_win(opponents_pick),
                invalid_mode => panic!("invalid move {}, expected X, Y or Z", invalid_mode),
            },
        };

        sum += total_score(opponents_pick, our_pick);
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
