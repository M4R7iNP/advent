use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::{fs::File, io::BufReader, process::ExitCode};

const TEXT_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> ExitCode {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    println!("Hello, world!");

    let mut sum = 0;
    let start = Instant::now();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut first_num = 0;
        let mut last_num = 0;
        for (i, c) in line.chars().enumerate() {
            if c >= '1' && c <= '9' {
                println!("c: {c} ({})", c.to_digit(10).unwrap());
                if first_num > 0 {
                    last_num = c.to_digit(10).unwrap();
                } else {
                    first_num = c.to_digit(10).unwrap();
                }
            } else {
                for (n, text_number) in TEXT_NUMBERS.iter().enumerate() {
                    let end = i + text_number.len();
                    if end <= line.len() && (line[i..end] == **text_number) {
                        // println!("found text_number: {text_number} ({n}) in {line}");
                        if first_num > 0 {
                            last_num = n as u32;
                        } else {
                            first_num = n as u32;
                        }
                    }
                }
            }
        }

        if last_num == 0 {
            last_num = first_num;
        }

        let number = first_num * 10 + last_num;
        println!("{line} => {number}");

        sum += number;
    }
    println!("Result: {sum}");
    println!("Took: {}ms", start.elapsed().as_millis());
    return ExitCode::SUCCESS;
}
