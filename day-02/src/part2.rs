use strum::{Display, EnumString};

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    Ok(input.lines().map(|line| {
        let game = line.split(':').nth(1).unwrap();
        min_for_game(game, Color::Red) *
            min_for_game(game, Color::Green) *
            min_for_game(game, Color::Blue)
    }).sum::<usize>().to_string())
}

fn min_for_game(game: &str, color: Color) -> usize {
    //split by draw
    game.split(';').map(|draw| {
        //split by color
        draw.split(',')
            //clean up
            .map(|cubes| cubes.trim())
            //filter by current searched color
            .filter(|cubes| {
                cubes.ends_with(color.to_string().as_str())
            })
            //get the amount of cubes
            .map(|color| {
                color.split(' ').next().unwrap().parse::<usize>().unwrap_or(0)
            })
            .next().unwrap_or(0)
    }).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }

    #[test]
    fn test_min_for_game() {
        //Game 1
        assert_eq!(4, min_for_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Color::Red));
        assert_eq!(2, min_for_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Color::Green));
        assert_eq!(6, min_for_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Color::Blue));
        //Game 2
        assert_eq!(1, min_for_game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Color::Red));
        assert_eq!(3, min_for_game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Color::Green));
        assert_eq!(4, min_for_game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Color::Blue));
        //Game 3
        assert_eq!(20, min_for_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Color::Red));
        assert_eq!(13, min_for_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Color::Green));
        assert_eq!(6, min_for_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Color::Blue));
        //Game 4
        assert_eq!(14, min_for_game("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Color::Red));
        assert_eq!(3, min_for_game("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Color::Green));
        assert_eq!(15, min_for_game("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Color::Blue));
        //Game 5
        assert_eq!(6, min_for_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Color::Red));
        assert_eq!(3, min_for_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Color::Green));
        assert_eq!(2, min_for_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Color::Blue));
    }
}