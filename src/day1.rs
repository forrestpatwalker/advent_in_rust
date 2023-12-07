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

--- Full problem: https://adventofcode.com/2023/day/1 ---
*/
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let path = Path::new("input_files/day1_input.txt");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut total_value = 0;

    for line in reader.lines() {

        // let mut digit_hash: HashMap<usize, i32> = HashMap::new();
        // let calibration_string = line.unwrap();
        // let mut char_indices: Vec<(usize, char)> = calibration_string.char_indices().collect();

        // for (index, ch) in char_indices {
        //     if ch.is_digit(10) {
        //         digit_hash
        //     }
        // }



        // let first_num = calibration_string.chars_indices().collect().iter().find(|&(index, c)| c.is_digit(10));
        // let last_num = calibration_string.chars().rev().find(|&(index, c)| c.is_digit(10));

        // let combined = format!("{}{}", first_num.unwrap().1, last_num.unwrap());

        // total_value += combined.parse::<i32>().unwrap();
    }

    println!("The sum of all of the calibration values is: {}", total_value);
}