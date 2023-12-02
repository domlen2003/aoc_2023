use crate::custom_error::AocError;

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    Ok(input.lines().enumerate().filter_map(|(i, line)| {
        if game_possible(line.split(':').nth(1).unwrap()) {
            Some(i + 1)
        } else {
            None
        }
    }).sum::<usize>().to_string())
}

fn game_possible(draws: &str) -> bool {
    draws.split(';').all(|draw| {
        draw.split(',')
            .map(|color| color.trim())
            .all(|color| {
                let amount = color.split(' ').next().unwrap().parse::<u32>().unwrap_or(0);
                if color.ends_with("red") {
                    amount <= 12
                } else if color.ends_with("green") {
                    amount <= 13
                } else if color.ends_with("blue") {
                    amount <= 14
                } else {
                    panic!("unknown color: {}", color)
                }
            })
    })
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
        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[test]
    fn test_game_possible() {
        assert_eq!(true, game_possible("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        assert_eq!(true, game_possible("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));
        assert_eq!(false, game_possible("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"));
        assert_eq!(false, game_possible("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"));
        assert_eq!(true, game_possible("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
    }
}