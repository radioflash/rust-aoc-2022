mod d1;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

pub fn solve_day(day: u8, part: Part, input: String) -> i32 {
    
    match day {
        1 => d1::solve(part, input),
        _ => panic!("Exercise day {} NYI", day),
    }
}