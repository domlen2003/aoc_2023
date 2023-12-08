use std::collections::HashMap;

use strum::{Display, EnumString};

use crate::custom_error::AocError;
use crate::custom_error::AocError::ParseError;

#[derive(Debug, PartialEq)]
pub(crate) struct Definition<'a> {
    pub nav: Vec<Direction>,
    pub network: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Direction {
    Left,
    Right,
}


pub(crate) fn parse(input: &str) -> Result<Definition, AocError> {
    let (nav, network) = input.split_once("\n\n").ok_or(ParseError("Invalid input".to_string()))?;
    let nav = nav.trim().chars().filter_map(|c| {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }).collect::<Vec<Direction>>();
    let network = network.lines().filter_map(|line| {
        if !(line.len() == 16 && &line[3..7] == " = (" && &line[10..12] == ", " && &line[15..] == ")") {
            eprintln!("Invalid line: {}", line);
            return None;
        }
        Some((&line[..3], (&line[7..10], &line[12..15])))
    }).collect::<HashMap<&str, (&str, &str)>>();
    Ok(Definition {
        nav,
        network,
    })
}

pub(crate) const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub(crate) const fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use crate::utils::Direction::{Left, Right};

    use super::*;

    #[test]
    fn test_parse() -> Result<(), AocError> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(Definition {
            nav: vec![Left, Left, Right],
            network: HashMap::from([
                ("AAA", ("BBB", "BBB")),
                ("BBB", ("AAA", "ZZZ")),
                ("ZZZ", ("ZZZ", "ZZZ")),
            ]),
        }, parse(input)?);
        Ok(())
    }
}