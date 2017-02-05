use std::char;

pub fn annotate(input: &[&str]) -> Vec<String> {
    let (rows, cols) = (input.len(), input[0].len());
    let board: Vec<char> = input.iter().flat_map(|r| r.chars()).collect();
    let mut result: Vec<String> = vec![];
    for row in 0..rows {
        let mut line = String::new();
        for col in 0..cols {
            // board[row][col] == board[row*cols+col]
            match board[row * cols + col] {
                '*' => line.push('*'),
                _ => {
                    let mut mines = 0;
                    if row > 0 {
                        // NW N NE
                        if col > 0 {
                            if board[(row - 1) * cols + (col - 1)] == '*' {
                                mines += 1; // NW
                            }
                        }
                        if board[(row - 1) * cols + col] == '*' {
                            mines += 1; // N
                        }
                        if col < cols - 1 {
                            if board[(row - 1) * cols + (col + 1)] == '*' {
                                mines += 1; // NE
                            }
                        }
                    }
                    if row < rows - 1 {
                        // SW S SE
                        if col > 0 {
                            if board[(row + 1) * cols + (col - 1)] == '*' {
                                mines += 1; // SW
                            }
                        }
                        if board[(row + 1) * cols + col] == '*' {
                            mines += 1; // S
                        }
                        if col < cols - 1 {
                            if board[(row + 1) * cols + (col + 1)] == '*' {
                                mines += 1; // SE
                            }
                        }
                    }
                    if col > 0 {
                        if board[row * cols + (col - 1)] == '*' {
                            mines += 1; // W
                        }
                    }
                    if col < cols - 1 {
                        if board[row * cols + (col + 1)] == '*' {
                            mines += 1; // E
                        }
                    }
                    if mines > 0 {
                        line.push(char::from_digit(mines, 10).unwrap());
                    } else {
                        line.push(' ');
                    }
                }
            }
        }
        result.push(line);
    }
    result
}