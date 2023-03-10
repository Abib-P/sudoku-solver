mod sudoku_parser;

use std::fs;
use std::env;
use crate::sudoku_parser::parse_sudoku;

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
    //print the sudoku
    print!("{:?}", sudoku);
    //solve the the sudoku
}
