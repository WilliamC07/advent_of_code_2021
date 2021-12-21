use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

struct Value {
    bit_string: String,
    invalid: bool,
}

pub fn run() {
    println!("Running Day 3 part 2:");

    let path = Path::new("src/day_3/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut values: Vec<Value> = lines
        .map(|line| Value { bit_string: line.unwrap(), invalid: false })
        .collect();

    // oxygen: most common stays
    let oxygen_value = get_value(&mut values, remove_most_common);

    // clear values
    for value in values.iter_mut() {
        value.invalid = false;
    }

    let carbon_value = get_value(&mut values, remove_least_common);
    println!("Answer day 3 part 2 = {} * {} = {}", oxygen_value, carbon_value, oxygen_value * carbon_value);
}

fn remove_most_common(current_char: char, most_common_char: char) -> bool {
    current_char != most_common_char
}

fn remove_least_common(current_char: char, most_common_char: char) -> bool {
    current_char == most_common_char
}

fn get_value(values: &mut Vec<Value>, should_remove: fn(char, char) -> bool) -> i32 {
    let mut remaining = values.len();
    let mut position: usize = 0;
    while remaining != 1 {
        // count most frequent in the current position
        let mut num_ones = 0;
        for value in values.iter_mut() {
            if !value.invalid && value.bit_string.as_bytes()[position] as char == '1' {
                num_ones += 1;
            }
        }

        let most_common_character = if num_ones >= remaining - num_ones { '1' } else { '0' };
        for value in values.iter_mut() {
            if !value.invalid &&
                should_remove(value.bit_string.as_bytes()[position] as char, most_common_character) {
                value.invalid = true;
                remaining -= 1;
            }
        }

        position += 1;
    }

    for value in values {
        if !value.invalid {
            return i32::from_str_radix(&value.bit_string, 2).unwrap();
        }
    }

    panic!("Fail to find");
}
