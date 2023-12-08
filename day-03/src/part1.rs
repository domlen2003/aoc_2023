use std::collections::BTreeMap;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq)]
enum Value {
    Symbol(char),
    Empty,
    Number(u8),
}

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
    dbg!(number_packs.clone());
    Ok(
        number_packs.iter().filter(|(_, points)| {
            points.iter().any(|(y, x)| {
                OFFSETS.iter().any(|(y_offset, x_offset)| {
                    map.get(&(y + y_offset, x + x_offset)).map_or(false, |value| {
                        if let Value::Symbol(_) = value {
                            true
                        } else {
                            false
                        }
                    })
                })
            })
        }).map(|(number, _)| number).sum::<u32>().to_string()
    )
}

fn get_number_packs(map: &BTreeMap<(i32, i32), Value>) -> Vec<(u32, Vec<(i32, i32)>)> {
    // filter all numbers and group them by horizontal adjacency
    map.iter().filter_map(|(index, value)| {
        if let Value::Number(number) = value {
            Some((index, *number))
        } else {
            None
        }
    }).fold(Vec::new(), |mut acc: Vec<Vec<((i32, i32), u8)>>, (index, number)| {
        if let Some(last) = acc.last_mut() {
            if let Some(((_, last_x), _)) = last.last() {
                if last_x + 1 == (*index).1 {
                    last.push((*index, number));
                    return acc;
                }
            }
        }
        acc.push(vec![(*index, number)]);
        acc
    }).iter().map(|pack| {
        let mut acc: u32 = 0;
        let mut points: Vec<(i32, i32)> = Vec::new();
        for (index, val) in pack {
            points.push(*index);
            acc = acc * 10 + (*val as u32);
        }
        (acc, points)
    }).collect::<Vec<(u32, Vec<(i32, i32)>)>>()
}

fn parse(input: &str) -> BTreeMap<(i32, i32), Value> {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            let value = match c {
                '.' => Value::Empty,
                value if value.is_ascii_digit() => Value::Number(value.to_digit(10).expect("Invalid digit") as u8),
                value => Value::Symbol(value),
            };
            ((y as i32, x as i32), value)
        })
    }).collect::<BTreeMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use crate::part1::Value::{Empty, Number, Symbol};

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

    #[test]
    fn test_parse() -> Result<(), AocError> {
        let input = "467..114..
...*......";

        assert_eq!(BTreeMap::from([
            //line 1
            ((0, 0), Number(4)),
            ((0, 1), Number(6)),
            ((0, 2), Number(7)),
            ((0, 3), Empty),
            ((0, 4), Empty),
            ((0, 5), Number(1)),
            ((0, 6), Number(1)),
            ((0, 7), Number(4)),
            ((0, 8), Empty),
            ((0, 9), Empty),
            // line 2
            ((1, 0), Empty),
            ((1, 1), Empty),
            ((1, 2), Empty),
            ((1, 3), Symbol('*')),
            ((1, 4), Empty),
            ((1, 5), Empty),
            ((1, 6), Empty),
            ((1, 7), Empty),
            ((1, 8), Empty),
            ((1, 9), Empty),
        ]), parse(input));
        Ok(())
    }

    #[test]
    fn test_get_number_packs() -> Result<(), AocError> {
        let input = "467..114..
...*......";
        let map = parse(input);
        assert_eq!(vec!(
            (467, vec![(0, 0), (0, 1), (0, 2)]),
            (114, vec![(0, 5), (0, 6), (0, 7)]),
        ), get_number_packs(&map));
        Ok(())
    }
}