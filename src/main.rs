mod d1;
mod input_fetcher;

fn solve_exercise(i: i32) {
    let input = input_fetcher::fetch(1, "53616c7465645f5fd2b00d1351048ded6ed85148811c17bfd2f81729e928fd838a035b378a7ec97f6b223b4e21c90c68330f08675a72a503bb8969ba2af31cfe");
    match i {
        1 => println!("{}", d1::solve(input)),
        _ => println!("Exercise {} NYI", i),
    }
}

fn main() {
    let exercise = match std::env::args().nth(1) {
        Some(s) => s.parse::<i32>().expect("Parsing failed"),
        None => 1,
    };
    assert!(
        exercise >= 1 && exercise <= 25,
        "Invalid exercise number; 1-25 is allowed"
    );

    println!("Solution for: {}", exercise);
    solve_exercise(exercise);
}
