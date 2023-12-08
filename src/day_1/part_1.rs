use crate::errors::AdventError;

pub fn _part_one(input: &str) -> Result<i32, AdventError> {
    let mut total_value = 0;

    for line in input.lines() {
        let first_num = line.chars().find(|c| c.is_digit(10)).expect("There should be a first digit");
        let second_num = line.chars().rev().find(|c| c.is_digit(10)).expect("There should be a second digit");

        let combined = format!("{first_num}{second_num}").parse::<i32>().expect("concatenated value should parse");

        total_value += combined;
    }

    Ok(total_value)
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