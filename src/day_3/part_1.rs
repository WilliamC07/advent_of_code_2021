use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run() {
    println!("Running Day 3 part 1:");

    let path = Path::new("src/day_3/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    const NUM_BITS: u32 = 12;
    let mut occurrences_of_one: [u32; NUM_BITS as usize] = [0; NUM_BITS as usize];

    for line in lines {
        if let Ok(bitstring) = line {
            for (position, value) in bitstring.chars().enumerate() {
                occurrences_of_one[position] += (value as u32) - '0' as u32;
            }
        }
    }

    let epsilon =
        occurrences_of_one
            .iter()
            .map(|val| if *val > 500 {0} else {1})
            .map(|val| val.to_string())
            .collect::<String>(); // least common
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    let gamma =
        occurrences_of_one
            .iter()
            .map(|val| if *val > 500 {1} else {0})
            .map(|val| val.to_string())
            .collect::<String>(); // most common
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();

    println!("Answer day 3 part 1: {}", gamma * epsilon);
}
