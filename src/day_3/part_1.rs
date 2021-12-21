use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run(){
    println!("Running Day 3 part 1:");

    let path = Path::new("src/day_1/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut previous_depth = 0;
    let mut count_decreased = 0;
    let lines = io::BufReader::new(file).lines();
    for (index, line) in lines.enumerate() {
        if let Ok(depth) = line {
            let depth = depth.parse::<u32>().unwrap();
            if index != 0 && previous_depth < depth{
                count_decreased += 1;
            }
            previous_depth = depth;
        }
    }

    println!("Answer day 3 part 1: {}", count_decreased);
}
