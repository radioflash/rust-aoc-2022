mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

mod parsing_tools;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

pub fn solve_day(day: u8, part: Part, input: &str) -> String {
    match day {
        1 => d1::solve(part, &input).to_string(),
        2 => d2::solve(part, &input).to_string(),
        3 => d3::solve(part, &input).to_string(),
        4 => d4::solve(part, &input).to_string(),
        5 => d5::solve(part, &input),
        6 => d6::solve(part, &input).to_string(),
        _ => panic!("Exercise day {} NYI", day),
    }
}
