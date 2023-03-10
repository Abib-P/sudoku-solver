mod sudoku_helper;

use std::fs;
use std::env;
use crate::sudoku_helper::{parse_sudoku, solve_sudoku};

fn main() {
    //get the arguments
    let args: Vec<String> = env::args().collect();
    //check if the user has provided a file
    if args.len() != 2 {
        panic!("Usage: sudoku <file>");
    }
    //read the file of sudoku
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
    //parse the sudoku
    let sudoku = parse_sudoku(&contents);
    //solve the the sudoku
    let solved_sudoku = solve_sudoku(sudoku);
    //print the solved sudoku
    for row in solved_sudoku {
        for col in row {
            print!("{} ", col);
        }
        println!("");
    }
}
