pub fn parse_sudoku(contents: &String) -> Vec<Vec<u8>> {
    let mut sudoku: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            if c == '.' {
                row.push(-1);
            } else {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }
        sudoku.push(row);
    }
    sudoku
}