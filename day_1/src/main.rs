use std::{fs::File, io::{BufReader, BufRead}};

fn main(){
    let file = File::open("input.txt").expect("No file found!");
    let reader: Vec<String> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();


    let mut block_calories = 0;
    let mut most_calories = 0;


    for line in reader {

        match line.parse::<i32>(){
            Ok(line_value) => {
                block_calories += line_value;
            },
            Err(e) => {
                if block_calories > most_calories {
                    most_calories = block_calories
                }

                block_calories = 0;
            }
        }
    }

    println!("The most calories carried by an elf is: {most_calories}")
}