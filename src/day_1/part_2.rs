use std::collections::{HashMap, BTreeMap};
use crate::errors::AdventError;

pub fn _part_two(input: &str) -> Result<i32, AdventError> {
    let mut total_value = 0;
    for line in input.lines() {
        let numbers:HashMap<&str, char> = HashMap::from([
            ("zero", '0'),
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
            ]);

        let mut digit_hash: BTreeMap<usize, char> = BTreeMap::new();
        let char_indices: Vec<(usize, char)> = line.char_indices().collect();

        for (index, ch) in char_indices {
            if ch.is_digit(10) {
                digit_hash.insert(index, ch);
            }
        }
        
        for (key, val) in numbers {
            let mut start = 0;
            while let Some(found) = line[start..].find(key) {
                digit_hash.insert(found + start, val);
                start += found + 1;
            }
        }

        let combined = format!("{}{}", digit_hash.iter().next().unwrap().1, digit_hash.iter().next_back().unwrap().1);
        total_value += combined.parse::<i32>().unwrap();
    }

    return Ok(total_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(Ok(281), _part_two(input))
    }

    #[test]
    fn full_test() {
        let input = include_str!("day1_input.txt");

        assert_eq!(Ok(54925), _part_two(input))
    }
}