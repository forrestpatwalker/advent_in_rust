/*
--- Advent of Code 2023 - Day 1: Trebuchet?! ---


Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, 
they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked 
when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why 
your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") 
when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf 
who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need 
to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

Your puzzle answer was 55172.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?


--- Full problem: https://adventofcode.com/2023/day/1 ---
*/
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::{HashMap, BTreeMap};

pub fn run() {
    let path = Path::new("input_files/day1_input.txt");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut total_value = 0;

    for line in reader.lines() {
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
        let calibration_string = line.unwrap();
        let char_indices: Vec<(usize, char)> = calibration_string.char_indices().collect();

        for (index, ch) in char_indices {
            if ch.is_digit(10) {
                digit_hash.insert(index, ch);
            }
        }
        
        for (key, val) in numbers {
            let mut start = 0;
            while let Some(found) = calibration_string[start..].find(key) {
                digit_hash.insert(found + start, val);
                start += found + 1;
            }
        }

        let combined = format!("{}{}", digit_hash.iter().next().unwrap().1, digit_hash.iter().next_back().unwrap().1);
        total_value += combined.parse::<i32>().unwrap();
    }

    println!("The sum of all of the calibration values is: {}", total_value);
}