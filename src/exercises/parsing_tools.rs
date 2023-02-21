// this module provides utility traits to string iterators for expecting/consuming tokens
/* example use:
let a = "these are 4 tokens";
let mut it = a.split(" ");
it.parse_literal("these")?;
it.parse_literal("are")?;
let cnt = it.parse_u8()?;
it.parse_literal("tokens")?;
*/
use anyhow::{anyhow, bail, Context, Result};

pub trait ParseU8: Iterator {
    fn parse_u8<'a>(&mut self) -> Result<u8>
    where
        Self: std::iter::Iterator<Item = &'a str>,
        Self: Sized,
    {
        try_and_consume_u8(self)
    }
}

pub trait ParseLiteral: Iterator {
    fn parse_literal<'a>(&mut self, expected_token: &str) -> Result<()>
    where
        Self: std::iter::Iterator<Item = &'a str>,
        Self: Sized,
    {
        try_and_consume_string_literal(self, expected_token)
    }
}

fn try_and_consume_string_literal<'a, I>(iter: &mut I, expected_token: &str) -> Result<()>
where
    I: std::iter::Iterator<Item = &'a str>,
{
    let token = iter
        .next()
        .with_context(|| format!("No \"{}\" token", expected_token))?;
    if token != expected_token {
        bail!("Did not get expected \"{expected_token}\" but \"{token}\" instead");
    }
    Ok(())
}

fn try_and_consume_u8<'a, I>(iter: &mut I) -> Result<u8>
where
    I: std::iter::Iterator<Item = &'a str>,
{
    let token: &str = iter.next().context("Missing expected u8 number")?;
    let num: u8 = token
        .parse()
        .with_context(|| anyhow!("Parsing failed, expected number, got {}", token))?;
    Ok(num)
}

impl<I: Iterator> ParseU8 for I {}
impl<I: Iterator> ParseLiteral for I {}

