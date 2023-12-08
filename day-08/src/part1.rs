use crate::custom_error::AocError;
use crate::utils::{Direction, parse};

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    let def = parse(input)?;
    let mut current = "AAA";
    let mut steps: u128 = 0;
    while current != "ZZZ" {
        let (left, right) = def.network.get(current).unwrap();
        if def.nav[(steps % def.nav.len() as u128) as usize] == Direction::Left {
            current = left;
        } else {
            current = right;
        }
        steps += 1;
    }
    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}