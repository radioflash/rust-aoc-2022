pub fn solve(input: String) -> i32 {
    return input
        .split("\n\n") // split into per-elf chunks
        .map(|s| {
            s.split("\n").fold(0, |acc, x| {
                match x {
                    "" => acc,
                    _ => acc + x.parse::<i32>().expect("invalid calorie number: ")
                }
            })
        }) // sum calories for each elf
        .max().expect("invalid input: no maximum"); // max value
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check() {
        let s = indoc! {"
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
        assert!(!s.contains(" "));
        assert_eq!(solve(s.to_string()), 24000);
    }
}
