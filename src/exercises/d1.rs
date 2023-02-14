use super::Part;

pub fn solve(part: Part, input: impl Into<String>) -> i32 {
    let s = input.into();
    let calories_per_elf = s
    .split("\n\n") // split into per-elf chunks
    .map(|s| {
        s.split("\n").fold(0, |acc, x| {
            match x {
                "" => acc,
                _ => acc + x.parse::<i32>().expect("invalid calorie number")
            }
        })
    }); // sum calories for each elf

    match part {
        Part::One => calories_per_elf.max().expect("Got no maximum value, invalid input?"),
        Part::Two => {
            let mut list: Vec<i32> = calories_per_elf.collect::<Vec<i32>>(); // collect entries
            list.sort_by(|a, b| b.cmp(a)); // get entries in reverse order
            list.truncate(3); // truncate to top 3
            list.iter().sum()
        },
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn part1() {
        assert!(!TEST_INPUT.contains(" "));
        assert_eq!(solve(TEST_INPUT.to_string(), 1), 24000);
    }
    #[test]
    fn part2() {
        assert_eq!(solve(TEST_INPUT.to_string(), 2), 45000);
    }
}
