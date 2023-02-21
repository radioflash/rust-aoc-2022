use anyhow::Context;
use indoc::indoc;

mod exercises;
mod input_fetcher;

fn main() {
    // argument parsing
    let arg1 = std::env::args().nth(1);
    let arg2 = std::env::args().nth(2);

    if arg1.is_none() || arg2.is_none() {
        println!(indoc! {"Run with 'cargo run <day> <part>' where:
            <day> is in [1-25]
            <part> is either 1 or 2
        "});
    }

    let day: u8 = match std::env::args().nth(1) {
        Some(s) => s
            .parse::<u8>()
            .with_context(|| {
                format!(
                    "Failed to parse first argument, got \"{}\", expected an integer in  [1, 25]",
                    s
                )
            })
            .expect("Parse error"),
        None => 1,
    };

    let part = match std::env::args().nth(2) {
        Some(part) => match part.as_str() {
            "1" => exercises::Part::One,
            "2" => exercises::Part::Two,
            invalid_part_value => panic!(
                "Invalid part specified, expected 1 or 2, got: {}",
                invalid_part_value
            ),
        },
        None => exercises::Part::One,
    };

    assert!(day >= 1 && day <= 25, "Invalid day number; 1-25 is allowed");

    // extract session ID from environment variable
    let session_id = std::env::var("AOC_SESSION")
        .expect("Need a session key for looking up the input as environment variable AOC_SESSION!");

    let input = input_fetcher::fetch(day, session_id.as_str());
    let solution = exercises::solve_day(day, part, input.as_str());

    println!(
        "Solution for exercise day {}, part {:?}:\n{}",
        day, part, solution
    );
}
