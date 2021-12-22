use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run(){
    println!("Running Day 5 part 2:");

    let path = Path::new("src/day_5/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut matrix = [[0; 1000]; 1000];
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            // sample: 720,475 -> 720,669

            // ["[720,475]", "[720,669]"
            let parts: Vec<String> = line
                .split(" ")
                .filter(|e| e.as_bytes()[0] != '-' as u8)
                .map(|e| e.to_string())
                .collect();

            let start: Vec<usize> = parts[0]
                .split(",")
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = parts[1]
                .split(",")
                .map(|e| e.parse::<usize>().unwrap())
                .collect();

            let (mut x0,mut y0, x1, y1) = (start[0], start[1], end[0], end[1]);

            while x0 != x1 || y0 != y1 {
                matrix[y0][x0] += 1;
                if x0 > x1 {
                    x0 -= 1;
                }else if x0 < x1 {
                    x0 += 1;
                }

                if y0 > y1 {
                    y0 -= 1;
                }else if y0 < y1 {
                    y0 += 1;
                }
            }

            // mark the end
            matrix[y1][x1] += 1;
        }
    }

    let mut count = 0;
    for row in 0..1000 {
        for col in 0..1000 {
            if matrix[row][col] >= 2 {
                count += 1;
            }
        }
    }

    println!("Answer day 1 part 2: {}", count);
}
