use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run() {
    println!("Running Day 1 part 2:");

    let path = Path::new("src/day_1/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut depths: Vec<u32> = Vec::new();

    for line in lines {
        if let Ok(depth) = line {
            let depth = depth.parse::<u32>().unwrap();
            depths.push(depth);
        }
    }

    // window is of size 3
    let depths = depths;
    let mut previous_depth: u32 = 0;
    let mut count_decreased = 0;
    for (i, depth) in depths.iter().enumerate() {
        if i == depths.len() - 2 {
            // [0, 1, 2, 3, 4, 5] len = 6
            //              ^stop
            break;
        }

        let current_window_depth: u32 = depth + depths[i + 1] + depths[i + 2];
        if i != 0 && current_window_depth > previous_depth {
            count_decreased += 1;
        }
        previous_depth = current_window_depth;
    }

    println!("Answer day 1 part 2: {}", count_decreased);
}
