use regex::Regex;
use std::{fs, time::Instant};

use crate::Answer;

static MATCHING: &str = r"((\d+) (\w+))";

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_2.txt").unwrap();
    let time_no_io = Instant::now();

    let re = Regex::new(MATCHING).unwrap();

    let lines = file.lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let parsed = lines.map(|line| parse_line(line, &re));

    let total = parsed.clone().enumerate().fold(0, |acc, (index, val)| {
        if val.red <= max_red && val.blue <= max_blue && val.green <= max_green {
            acc + index + 1
        } else {
            acc
        }
    });

    let powers: i32 = parsed.map(|val| val.red * val.blue * val.green).sum();

    let part_1 = total.to_string();
    let part_2 = powers.to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 2,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}

fn parse_line(line: &str, re: &Regex) -> Rgb {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    re.captures_iter(line).for_each(|re_match| {
        let colour = &re_match[3];
        let value = re_match[2].parse::<i32>().unwrap();

        match colour {
            "red" => red = red.max(value),
            "green" => green = green.max(value),
            "blue" => blue = blue.max(value),
            _ => (),
        }
    });

    Rgb { red, blue, green }
}

#[derive(Debug)]
struct Rgb {
    red: i32,
    blue: i32,
    green: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let re = Regex::new(MATCHING).unwrap();
        let line = "Game 3: 12 red, 18 blue, 3 green; 14 red, 4 blue, 2 green; 4 green, 15 red";
        let rgb = parse_line(line, &re);

        assert_eq!(rgb.red, 15);
        assert_eq!(rgb.green, 4);
        assert_eq!(18, 18);
    }
}
