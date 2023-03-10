pub fn parse_sudoku(contents: &String) -> Vec<Vec<u8>> {
    let mut sudoku: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            if c == '.' {
                row.push(0);
            } else {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }
        sudoku.push(row);
    }
    sudoku
}

pub fn solve_sudoku(unsolve_sudoku: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    recursive_sudo_solver(unsolve_sudoku.clone(), 0, 0)
}

fn recursive_sudo_solver(sudoku: Vec<Vec<u8>>, x: u8, y: u8) -> Vec<Vec<u8>> {
    let mut res_sudoku = sudoku.clone();
    if sudoku[x as usize][y as usize] == 0 {
        // solve this case
        for i in 1..10 {
            if is_valid(sudoku.clone(), x, y, i) {
                res_sudoku[x as usize][y as usize] = i;
                let (nx, ny) = next_not_solved(res_sudoku.clone(), x, y);
                // stop si le next == 0
                if nx == 0 && ny == 0 {
                    return res_sudoku.clone();
                }
                res_sudoku = recursive_sudo_solver(res_sudoku.clone(), nx, ny);
                if res_sudoku[nx as usize][ny as usize] != 0 {
                    break;
                }
                res_sudoku[x as usize][y as usize] = 0;
            }
        }
    }
    // if not solved, return the original sudoku
    if res_sudoku[x as usize][y as usize] == 0 {
        return sudoku.clone();
    }
    let (nx, ny) = next_not_solved(res_sudoku.clone(), x, y);
    // stop si le next == 0
    if nx == 0 && ny == 0 {
        return res_sudoku.clone();
    }
    res_sudoku = recursive_sudo_solver(res_sudoku.clone(), nx, ny);
    if res_sudoku[nx as usize][ny as usize] != 0 {
        return res_sudoku.clone();
    }
    sudoku.clone()
}

fn is_valid(sudoku: Vec<Vec<u8>>, x: u8, y: u8, val: u8) -> bool {
    // check row
    for i in 0..9 {
        if sudoku[x as usize][i] == val {
            return false;
        }
    }
    // check column
    for i in 0..9 {
        if sudoku[i][y as usize] == val {
            return false;
        }
    }
    // check box
    let x0 = (x / 3) * 3;
    let y0 = (y / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku[x0 as usize + i][y0 as usize + j] == val {
                return false;
            }
        }
    }
    true
}

fn next_not_solved(sudoku: Vec<Vec<u8>>, x: u8, y: u8) -> (u8, u8) {
    let mut nx = x as u8;
    let mut ny = y as u8;
    ny += 1;
    if ny == 9 {
        ny = 0;
        nx += 1;
    }
    if nx == 9 {
        return (0, 0);
    }
    if sudoku[nx as usize][ny as usize] != 0 {
        return next_not_solved(sudoku, nx, ny);
    }
    return (nx, ny);
}