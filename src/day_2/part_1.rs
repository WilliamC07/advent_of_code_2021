use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run(){
    println!("Running Day 2 part 1:");

    let path = Path::new("src/day_2/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut horizontal = 0;
    let mut depth = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut parts = line.split(" ");
            let instruction = match parts.next() {
                Some(instruction) => instruction,
                None => panic!("Bad iterator"),
            };
            let factor = match parts.next() {
                Some(factor) => factor,
                None => panic!("Bad iterator"),
            };
            let factor = match factor.parse::<i32>() {
                Ok(factor) => factor,
                Err(why) => panic!("Fail to parse: {}", why),
            };

            let instruction_char = instruction.chars().next().unwrap();
            match instruction_char {
                'f' => horizontal += factor,
                'd' => depth += factor,
                'u' => depth -= factor,
                _ => panic!("Unexpected char {}", instruction_char),
            }
        }
    }

    println!("Answer day 2 part 1: {}", horizontal * depth);
}
