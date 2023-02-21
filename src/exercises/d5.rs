use anyhow::{anyhow, bail, Context, Result};

use super::parsing_tools::{ParseLiteral, ParseU8};
use super::Part;

fn parse_list_of_stacks(s: &str) -> Result<u8> {
    let mut stack_cnt: u8 = 0;

    for b in s.bytes() {
        match b {
            x if x.is_ascii_whitespace() => {
                continue;
            }
            x if x.is_ascii_digit() => {
                stack_cnt += 1;

                let i = (x - b'0') as u8;
                if i != stack_cnt {
                    bail!("Nonconsecutive value: {}", i);
                }
            }
            _ => bail!("Non-integer value: {}", b.to_string()),
        }
    }

    Ok(stack_cnt)
}

/* Each u8 is a crate.
 * Each Vec<u8> is a stack of crates (first item is the bottommost crate)
 * We return a vector of crate stacks, first entry is obviously the first crate stack.
 *
 * To make things easier we parse in reverse order (last line first).
 */
fn parse_crates(s: &str) -> Result<Vec<Vec<u8>>> {
    let mut it = s.rsplit('\n');

    let stack_list_str = it
        .next()
        .ok_or_else(|| anyhow!("Missing stacklist input"))?;
    let stack_cnt = parse_list_of_stacks(stack_list_str)?;

    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stack_cnt as usize];

    for (height, l) in it.enumerate() {
        for (i, chunk) in l.as_bytes().chunks(4).enumerate() {
            // we ignore the 4th chunk element because that is only an (optional) space to separate the stacks
            if i > stack_cnt as usize {
                return Err(anyhow!(
                    "Too many crate entries at height {}, epxected no more than {}",
                    height,
                    stack_cnt
                ));
            }

            if chunk.starts_with(b"   ") {
                continue;
            }
            if chunk.len() < 3 {
                return Err(anyhow!("expected <crate> (\"[X]\") or <empty> (\"    \"), got: \"{}\" for stack {} at height {}", std::str::from_utf8(chunk).unwrap(), i, height));
            }

            if chunk[0] == b'[' && chunk[2] == b']' {
                stacks[i].push(chunk[1]);
            }
        }
    }

    Ok(stacks)
}

struct Move {
    crate_cnt: u8,
    from_stack: u8,
    to_stack: u8,
}

fn parse_move(s: &str) -> Result<Move> {
    let mut iter = s.split(" ");

    iter.parse_literal("move")?;
    let cnt = iter.parse_u8()?;
    iter.parse_literal("from")?;
    let from = iter.parse_u8()?;
    iter.parse_literal("to")?;
    let to = iter.parse_u8()?;

    Ok(Move {
        crate_cnt: cnt,
        from_stack: from,
        to_stack: to,
    })
}

pub fn solve(part: Part, input: &str) -> String {
    let mut it = input.split("\n\n");

    // find out initial crate stack setup
    let mut stacks = parse_crates(it.next().expect("Expected initial crate setup"))
        .expect("cratestack parsing failed");
    let moves = it.next().expect("Expected crate moves").trim_end();

    // move crates around
    for (move_num, m_str) in moves.split('\n').enumerate() {
        let m = parse_move(m_str).expect("Got no move");

        match part {
            Part::One => {
                for _ in 0..m.crate_cnt {
                    let c = stacks[(m.from_stack - 1) as usize]
                        .pop()
                        .with_context(|| {
                            anyhow!(
                                "Not enough crates to take {} in move {}",
                                m.crate_cnt,
                                move_num
                            )
                        })
                        .unwrap();
                    stacks[(m.to_stack - 1) as usize].push(c);
                }
            }
            Part::Two => {
                let mut crates = {
                    let origin_stack = &mut stacks[(m.from_stack - 1) as usize];
                    origin_stack.split_off(origin_stack.len() - m.crate_cnt as usize)
                };
                assert!(
                    crates.len() == m.crate_cnt as usize,
                    "Unable to move {} crates from stack {} to stack {} (move {})",
                    m.crate_cnt,
                    m.from_stack,
                    m.to_stack,
                    move_num
                );

                let target_stack = &mut stacks[(m.to_stack - 1) as usize];
                target_stack.append(&mut crates);
            }
        }
    }

    // extract top crate name for each stack
    let mut top_crates: String = String::from("");
    for s in stacks {
        top_crates.push(match s {
            mut x if !x.is_empty() => x.pop().unwrap() as char,
            _ => ' ',
        });
    }

    top_crates
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
        1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_parse_stacklist() {
        assert_eq!(parse_list_of_stacks("1   2   3 ").unwrap(), 3)
    }

    #[test]
    fn test_parse_crates() {
        let x = parse_crates(TEST_INPUT.split("\n\n").next().unwrap());
        println!("{:?}", x);
        assert!(x.is_ok());
    }

    #[test]
    fn test_parse_move() {
        let moves = TEST_INPUT.split("\n\n").skip(1).next().unwrap();
        let m = parse_move(moves.split("\n").next().unwrap()).unwrap();
        println!(
            "cnt: {}, from: {}, to: {}",
            m.crate_cnt, m.from_stack, m.to_stack
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, &TEST_INPUT), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(solve(Part::Two, &TEST_INPUT), "MCD");
    }
}
