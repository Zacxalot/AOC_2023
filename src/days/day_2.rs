use itertools::Itertools;
use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_2.txt").unwrap();
    let time_no_io = Instant::now();

    let lines = file.lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let total = lines
        .map(parse_line)
        .enumerate()
        .fold(0, |acc, (index, val)| {
            if val.red <= max_red && val.blue <= max_blue && val.green <= max_green {
                println!("{:?} {:?}", index + 1, val);

                acc + index + 1
            } else {
                acc
            }
        });

    let part_1 = total.to_string();
    let part_2 = "2".to_string();

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

enum Colour {
    Red,
    Blue,
    Green,
}

fn parse_line(line: &str) -> RGB {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    line.split(": ")
        .nth(1)
        .unwrap()
        .split(';')
        .for_each(|pull| {
            let mut pull_red = 0;
            let mut pull_blue = 0;
            let mut pull_green = 0;

            pull.split(',').for_each(|pair| {
                let mut split = pair.trim().split(' ');
                let val = split.next().unwrap();
                println!("PAIR {pair}");
                // println!("Val {val} colour - {:?}", split.next().unwrap());
                let colour = match_colour(split.next().unwrap());

                match colour {
                    Colour::Red => pull_red += val.parse::<i32>().unwrap(),
                    Colour::Blue => pull_blue += val.parse::<i32>().unwrap(),
                    Colour::Green => pull_green += val.parse::<i32>().unwrap(),
                }
            });

            println!("LINE ---- {pull}");

            red = red.max(pull_red);
            green = green.max(pull_green);
            blue = blue.max(pull_blue);
        });

    RGB { red, blue, green }
}

#[derive(Debug)]
struct RGB {
    red: i32,
    blue: i32,
    green: i32,
}

fn match_colour(colour: &str) -> Colour {
    match colour {
        "red" => Colour::Red,
        "blue" => Colour::Blue,
        "green" => Colour::Green,
        "red;" => Colour::Red,
        "blue;" => Colour::Blue,
        "green;" => Colour::Green,
        "red," => Colour::Red,
        "blue," => Colour::Blue,
        "green," => Colour::Green,
        _ => panic!("Invalid colour"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 3: 12 red, 18 blue, 3 green; 14 red, 4 blue, 2 green; 4 green, 15 red";
        let rgb = parse_line(line);

        println!("{:?}", rgb);

        assert_eq!(rgb.red, 15);
        assert_eq!(rgb.green, 4);
        assert_eq!(18, 18);
    }
}
