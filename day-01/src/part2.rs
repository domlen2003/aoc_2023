use crate::custom_error::AocError;

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    Ok(input.lines().map(process_line).sum::<u32>().to_string())
}

fn process_line(line: &str) -> u32 {
    let mut numbers = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let chars: &[char] = &reduced_line.chars().collect::<Vec<char>>();
        match chars {
            ['o', 'n', 'e', ..] => Some(1),
            ['t', 'w', 'o', ..] => Some(2),
            ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
            ['f', 'o', 'u', 'r', ..] => Some(4),
            ['f', 'i', 'v', 'e', ..] => Some(5),
            ['s', 'i', 'x', ..] => Some(6),
            ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
            ['e', 'i', 'g', 'h', 't', ..] => Some(8),
            ['n', 'i', 'n', 'e', ..] => Some(9),
            _ => reduced_line.chars().next().unwrap().to_digit(10),
        }
    });
    let first = numbers.next().unwrap_or(0);
    first * 10 + numbers.last().unwrap_or(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_test() {
        assert_eq!(29, process_line("two1nine"));
        assert_eq!(83, process_line("eightwothree"));
        assert_eq!(13, process_line("abcone2threexyz"));
        assert_eq!(24, process_line("xtwone3four"));
        assert_eq!(42, process_line("4nineeightseven2"));
        assert_eq!(14, process_line("zoneight234"));
        assert_eq!(76, process_line("7pqrstsixteen"));
        assert_eq!(51, process_line("fivezg8jmf6hrxnhgxxttwoneg"));
    }

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}