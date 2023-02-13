use anyhow::Context;
use indoc::indoc;

mod d1;
mod input_fetcher;

fn solve_exercise(exercise: i32, part: i32) {
    let input = input_fetcher::fetch(1, "53616c7465645f5fd2b00d1351048ded6ed85148811c17bfd2f81729e928fd838a035b378a7ec97f6b223b4e21c90c68330f08675a72a503bb8969ba2af31cfe");
    match exercise {
        1 => println!("{}", d1::solve(input, part)),
        _ => println!("Exercise {} NYI", exercise),
    }
}

fn main() {
    let arg1 = std::env::args().nth(1);
    let arg2 = std::env::args().nth(2);

    if arg1.is_none() || arg2.is_none() {
        println!(indoc! {"Run with 'cargo run <exercise> <part>' where:
            <exercise> is in [1-25]
            <part> is either 1 or 2
        "});
    }

    let exercise = match std::env::args().nth(1) {
        Some(s) => s.parse::<i32>().with_context(|| format!("Failed to parse first argument, got \"{}\", expected an integer in  [1, 25]", s)).expect("Parse error"),
        None => 1,
    };
    let part = match std::env::args().nth(2) {
        Some(s) => s.parse::<i32>().with_context(|| format!("Failed to parse first argument, got \"{}\", expected an integer in  [1, 25]", s)).expect("Parse error"),
        None => 1,
    };

    assert!(
        exercise >= 1 && exercise <= 25,
        "Invalid exercise number; 1-25 is allowed"
    );
    assert!(
        part >= 1 && part <= 2,
        "Invalid part number; 1-2 is allowed"
    );

    println!("Solution for exercise {}, part {}", exercise, part);
    solve_exercise(exercise, part);
}
