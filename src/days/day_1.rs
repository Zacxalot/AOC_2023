use itertools::Itertools;
use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_1.txt").unwrap();
    let time_no_io = Instant::now();

    let lines = file.lines();

    let part_1 = lines.clone().map(parse_line).sum::<usize>().to_string();
    let part_2 = lines
        .clone()
        .map(parse_stringy_line)
        .sum::<usize>()
        .to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 1,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}

fn parse_line(line: &str) -> usize {
    let mut first: usize = 11;
    let mut last: usize = 11;

    line.chars().for_each(|c| {
        if c.is_ascii_digit() {
            if first == 11 {
                first = c.to_digit(10).unwrap() as usize;
                last = first;
            } else {
                last = c.to_digit(10).unwrap() as usize;
            }
        }
    });

    first * 10 + last
}

const STRING_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_stringy_line(line: &str) -> usize {
    let mut first: usize = 11;
    let mut last: usize = 11;

    line.chars().enumerate().for_each(|(pos, c)| {
        if c.is_ascii_digit() {
            if first == 11 {
                first = c.to_digit(10).unwrap() as usize;
                last = first;
            } else {
                last = c.to_digit(10).unwrap() as usize;
            }
        } else {
            let next = line
                .chars()
                .skip(pos)
                .take_while(|c| !c.is_ascii_digit())
                .collect::<String>();

            STRING_DIGITS.iter().enumerate().for_each(|(i, s)| {
                if next.starts_with(s) {
                    if first == 11 {
                        first = i;
                        last = first;
                    } else {
                        last = i;
                    }
                }
            });
        }
    });

    first * 10 + last
}

#[cfg(test)]
mod tests {}
