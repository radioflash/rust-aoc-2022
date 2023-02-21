use super::Part;

fn contains_duplicates(slice: &[u8]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i-1]) {
            return true;
        }
    }
    false
}

pub fn solve(part: Part, input: &str) -> usize {
    let bytes = input.as_bytes();
    let marker_length = match part {
        Part::One => 4,
        Part::Two => 14,
    };

    for i in marker_length..bytes.len() {
        if !contains_duplicates(&bytes[(i-marker_length)..i]) {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "};

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, &TEST_INPUT), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(solve(Part::Two, &TEST_INPUT), 19);
    }
}
