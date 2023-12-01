use crate::custom_error::AocError;

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    //map each line to first and last digit as a number
    Ok(input.lines().map(|line| {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
        let first = numbers.next().unwrap_or(0);
        first * 10 + numbers.last().unwrap_or(first)
    }).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}