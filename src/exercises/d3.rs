use super::Part;

fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => 1 + item - b'a',
        b'A'..=b'Z' => 27 + item - b'A',
        _ => panic!("unexpected input byte {item}, expected something in a-z | A-Z"),
    }
}

pub fn solve(part: Part, input: &str) -> i32 {
    let mut sum: i32 = 0;

    match part {
        Part::One => {
            for l in input.split('\n') {
                if l.is_empty() {
                    continue; //skip empty line which may occur at the end
                }

                assert!(l.len() % 2 == 0);
                let compartment_size = l.len() / 2;
                let mut compartment2: Vec<u8> = l.bytes().skip(compartment_size).collect();
                compartment2.sort();

                for byte in l.bytes().take(compartment_size) {
                    if compartment2.binary_search(&byte).is_ok() {
                        sum += priority(byte) as i32;
                        break;
                    }
                }
            }
        }
        Part::Two => {
            for group in input.split('\n').collect::<Vec<&str>>().chunks(3) {
                if group[0].is_empty() {
                    continue; //skip empty line which may occur at the end
                }

                let mut r2: Vec<u8> = group[1].bytes().collect();
                let mut r3: Vec<u8> = group[2].bytes().collect();
                r2.sort();
                r3.sort();

                for byte in group[0].bytes() {
                    if r2.binary_search(&byte).is_ok() && r3.binary_search(&byte).is_ok() {
                        sum += priority(byte) as i32;
                        break;
                    }
                }
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, TEST_INPUT), 157);
    }

    #[test]
    fn part2() {
        assert!("äö∈".chars().skip(2).next().unwrap() == '∈');
        assert_eq!(solve(Part::Two, TEST_INPUT), 70);
    }
}
