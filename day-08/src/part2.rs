use crate::custom_error::AocError;
use crate::custom_error::AocError::ProcessError;
use crate::utils::{Direction, lcm, parse};

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    let def = parse(input)?;
    def.network.keys().filter(|key| {
        key.ends_with('A')
    }).map(|s| {
        let mut current = s;
        let mut steps = 0;
        while !current.ends_with('Z') {
            let (left, right) = def.network.get(current).unwrap();
            if def.nav[steps % def.nav.len()] == Direction::Left {
                current = left;
            } else {
                current = right;
            }
            steps += 1;
        }
        steps
    }).reduce(lcm).map(|n| {
        n.to_string()
    }).ok_or(ProcessError("No path found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}