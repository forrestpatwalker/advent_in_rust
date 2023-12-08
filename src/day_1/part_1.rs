use crate::errors::AdventError;

pub fn _part_one(input: &str) -> Result<i32, AdventError> {
    let output = input
    .lines()
    .map(|line| {
        let mut it = line.chars().filter_map(|character| {
            character.to_digit(10)
        });

        let first = it.next().expect("should be a number");

        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}")
        }
        .parse::<i32>()
        .expect("Should be a valid number")

    })
    .sum::<i32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(Ok(142), _part_one(input))
    }

    #[test]
    fn full_test() {
        let input = include_str!("day1_input.txt");

        assert_eq!(Ok(55172), _part_one(input))
    }
}