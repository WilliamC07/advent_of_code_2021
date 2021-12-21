use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::VecDeque;

const SIDE_LEN: usize = 5;

struct Board {
    board: Vec<Vec<i32>>,
    completed: bool
}

pub fn run() {
    println!("Running Day 4 part 1 and part 2:");

    // reading file boilerplate
    let path = Path::new("src/day_4/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());

    // reading in number into a queue
    let mut numbers_chosen: VecDeque<i32> = VecDeque::new();
    if let Some(numbers) = lines.next() {
        numbers_chosen = numbers.split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>();
    }

    let mut boards: Vec<Board> = Vec::new();
    // create boards
    lines.next(); // consume the new line
    let mut board: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            // new board
            boards.push(Board{ board, completed: false});
            board = Vec::new();
        } else {
            board.push(line
                .split(" ")
                .map(|num_str| num_str.trim())
                .filter(|num_str| !num_str.is_empty())
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect());
        }
    }

    // add the last board
    boards.push(Board {board, completed: false});

    // play the bingo game
    let mut found_first_winner = false;
    let mut previous_winning_score = 0;
    while let Some(num) = numbers_chosen.pop_front() {
        for board in boards.iter_mut() {
            // mark if we found as -1
            for r in 0..SIDE_LEN {
                for c in 0..SIDE_LEN {
                    if board.board[r][c] == num {
                        board.board[r][c] = -1;
                    }
                }
            }

            if check_winner(board) {
                // found a winning board
                board.completed = true;

                let mut sum = 0;
                for r in 0..SIDE_LEN {
                    for c in 0..SIDE_LEN {
                        if board.board[r][c] != -1 {
                            sum += board.board[r][c];
                        }
                    }
                }

                let score = sum * num;
                if !found_first_winner {
                    println!("Answer day 4 part 1: {}", score);
                    found_first_winner = true;
                }

                previous_winning_score = score; // to find last winning board score
            }
        }

    }

    println!("Answer day 4 part 2: {}", previous_winning_score);
}

fn check_winner(board: &mut Board) -> bool {
    if board.completed {
        return false;
    }

    for i in 0..SIDE_LEN {

        // row by row
        let mut is_filled = true;
        for c in 0..SIDE_LEN {
            if board.board[i][c] != -1 {
                is_filled = false;
                break;
            }
        }

        if is_filled {
            return true;
        }

        // column by column
        let mut is_filled = true;
        for r in 0..SIDE_LEN {
            if board.board[r][i] != -1 {
                is_filled = false;
            }
        }

        if is_filled {
            return true;
        }
    }

    return false;
}
