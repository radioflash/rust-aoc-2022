use super::Part;
use std::ops::RangeInclusive;

fn parse_range(s: &str) -> RangeInclusive<i32> {
    let mut bounds = s.split("-");
    let a = bounds
        .next()
        .expect("missing lower bound")
        .parse::<i32>()
        .expect("parsing lower bound failed");
    let b = bounds
        .next()
        .expect("missing upper bound")
        .parse::<i32>()
        .expect("parsing upper bound failed");

    a..=b
}

fn either_is_common_superset(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    (r1.contains(r2.start()) && r1.contains(r2.end()))
        || (r2.contains(r1.start()) && r2.contains(r1.end()))
}

fn are_overlapping(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

pub fn solve(part: Part, input: &str) -> i32 {
    let mut cnt: i32 = 0;

    for l in input.trim().split("\n") {
        let mut ranges = l.split(",");
        let r1 = parse_range(ranges.next().expect("missing range"));
        let r2 = parse_range(ranges.next().expect("missing range"));

        match part {
            Part::One => {
                if either_is_common_superset(&r1, &r2) {
                    cnt += 1;
                }
            }
            Part::Two => {
                if are_overlapping(&r1, &r2) {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, &TEST_INPUT), 2);
    }

    #[test]
    fn part2() {
        let r1 = parse_range("5-7");
        let r2 = parse_range("7-9");
        assert!(are_overlapping(&r1, &r2));
        assert_eq!(solve(Part::Two, &TEST_INPUT), 4);
    }
}
