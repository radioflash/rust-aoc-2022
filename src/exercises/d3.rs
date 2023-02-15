use super::Part;

fn priority(item: char) -> u8 {
    match item {
        'a'..='z' => 1 + item as u8 - 'a' as u8,
        'A'..='Z' => 27 + item as u8 - 'A' as u8,
        _ => panic!("unexpected input character {item}, expected something in a-z | A-Z"),
    }
}

pub fn solve(part: Part, input: impl Into<String>) -> i32 {
    let s = input.into();

    let mut sum: i32 = 0;

    match part {
        Part::One => {
            for l in s.split('\n') {
                if l.is_empty() {
                    continue; //skip empty line which may occur at the end
                }
        
                let mut rucksack: Vec<char> = l.chars().collect();
                assert!(rucksack.len() % 2 == 0);
                let campartment_size = rucksack.len() / 2;
        
                // sort whats gonna be the first compartment
                (&mut rucksack[..campartment_size]).sort();
        
                let compartment1 = &rucksack[..campartment_size];
                let compartment2 = &rucksack[campartment_size..];
        
                for c in compartment2.iter() {
                    if compartment1.binary_search(c).is_ok() {
                        sum += priority(*c) as i32;
                        break;
                    }
                }
            }
        },
        Part::Two => {
            for group in s.split('\n').collect::<Vec<&str>>().chunks(3) {
                if group[0].is_empty() {
                    continue; //skip empty line which may occur at the end
                }

                let r1: Vec<char> = group[0].chars().collect();
                let mut r2: Vec<char> = group[1].chars().collect();
                let mut r3: Vec<char> = group[2].chars().collect();
                r2.sort();
                r3.sort();

                for c in r1.iter() {
                    if r2.binary_search(c).is_ok() && r3.binary_search(c).is_ok() {
                        sum += priority(*c) as i32;
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
        assert_eq!(solve(Part::Two, TEST_INPUT), 70);
    }
}
