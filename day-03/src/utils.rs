use std::collections::BTreeMap;

use crate::utils::Value::{Empty, Number, Symbol};

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Symbol(char),
    Empty,
    Number(u8),
}

pub(crate) fn get_number_packs(map: &BTreeMap<(i32, i32), Value>) -> Vec<(u32, Vec<(i32, i32)>)> {
    // filter all numbers and group them by horizontal adjacency
    map.iter().filter_map(|(index, value)| {
        if let Number(number) = value {
            Some((index, *number))
        } else {
            None
        }
    }).fold(Vec::new(), |mut acc: Vec<Vec<((i32, i32), u8)>>, (index, number)| {
        if let Some(last) = acc.last_mut() {
            if let Some(((_, last_x), _)) = last.last() {
                if last_x + 1 == index.1 {
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

pub(crate) fn parse(input: &str) -> BTreeMap<(i32, i32), Value> {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            let value = match c {
                '.' => Empty,
                value if value.is_ascii_digit() => Number(value.to_digit(10).expect("Invalid digit") as u8),
                value => Symbol(value),
            };
            ((y as i32, x as i32), value)
        })
    }).collect::<BTreeMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use crate::utils::Value::{Empty, Number, Symbol};

    use super::*;

    #[test]
    fn test_parse() {
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
    }

    #[test]
    fn test_get_number_packs() {
        let input = "467..114..
...*......";
        let map = parse(input);
        assert_eq!(vec!(
            (467, vec![(0, 0), (0, 1), (0, 2)]),
            (114, vec![(0, 5), (0, 6), (0, 7)]),
        ), get_number_packs(&map));
    }
}