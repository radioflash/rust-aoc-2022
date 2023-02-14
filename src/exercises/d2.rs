use super::Part;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}
use Shape::{Rock, Paper, Scissors};

fn score(opp: Shape, us: Shape) -> i32 {
    match (us, opp) { // reverse us/opponent for readability
        (Rock, Scissors)      => 0 + 1,
        (Rock, Rock)          => 3 + 1,
        (Rock, Paper)         => 6 + 1,
        (Paper, Rock)         => 0 + 2,
        (Paper, Paper)        => 3 + 2,
        (Paper, Scissors)     => 6 + 2,
        (Scissors, Paper)     => 6 + 3,
        (Scissors, Scissors)  => 3 + 3,
        (Scissors, Rock)      => 0 + 3,
    }
}

pub fn solve(part: Part, input: impl Into<String>) -> i32 {

    match part {
        Part::One => {
            let s = input.into();
            let mut sum: i32 = 0;
            for l in s.split('\n') {
                if l.is_empty() {
                    break;
                }
        
                let mut moves = l.split(' ');
                let opp = match moves.nth(0).expect("expected opponent move") {
                    "A" => Rock,
                    "B" => Paper,
                    "C" => Scissors,
                    invalid_mode => panic!("invalid move {}, expected A, B or C", invalid_mode),
                };
                let us = match moves.nth(0).expect("expected our move") {
                    "X" => Rock,
                    "Y" => Paper,
                    "Z" => Scissors,
                    invalid_mode => panic!("invalid move {}, expected X, Y or Z", invalid_mode),
                };
        
                sum += score(opp, us);        
            }
            return sum
        },
        Part::Two => todo!(),
    }
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
        //assert_eq!(solve(TEST_INPUT.to_string(), Part::One), 45000);
    }
}
