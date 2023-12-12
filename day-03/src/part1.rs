use crate::custom_error::AocError;
use crate::utils::{get_number_packs, parse, Value::Symbol};

const OFFSETS: [(i32, i32); 8] = [
    // above
    (-1, 0),
    (-1, 1),
    (-1, -1),
    // below
    (1, 0),
    (1, 1),
    (1, -1),
    // left and right
    (0, 1),
    (0, -1),
];

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    let map = parse(input);
    let number_packs = get_number_packs(&map);
    Ok(
        number_packs.iter().filter(|(_, points)| {
            points.iter().any(|(y, x)| {
                OFFSETS.iter().any(|(y_offset, x_offset)| {
                    map.get(&(y + y_offset, x + x_offset))
                        .map_or(false, |value| matches!(value,Symbol(_)))
                })
            })
        }).map(|(number, _)| number).sum::<u32>().to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}