// CSES 1624: Chessboard and Queens
// Category: Introductory Problems
// https://cses.fi/problemset/task/1624

use std::io::Write;

/// Is the square (row, col) safe from every queen placed in rows above?
fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
    for r in 0..row {
        // A queen `d` rows above attacks straight down, or along a diagonal
        // that shifts one column per row: it must sit at col, col-d, or col+d.
        let d = row - r;

        let same_column = board[r][col] == 'Q';
        let upper_left = col >= d && board[r][col - d] == 'Q';
        let upper_right = col + d < n && board[r][col + d] == 'Q';

        if same_column || upper_left || upper_right {
            return false;
        }
    }
    true
}

fn recurse(board: &mut Vec<Vec<char>>, row: usize, n: usize, res: &mut i64) {
    if row == n {
        *res += 1;
        return;
    }

    for col in 0..n {
        let free = board[row][col] == '.';
        if free && is_safe(board, row, col, n) {
            board[row][col] = 'Q';
            recurse(board, row + 1, n, res);
            board[row][col] = '.';
        }
    }
}

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    // the board is 8 * 8 we just need to take input.

    let mut board = Vec::new();
    for _ in 0..8 {
        let line = sc.line().chars().collect::<Vec<char>>();
        board.push(line);
    }

    let mut res = 0;
    recurse(&mut board, 0, 8, &mut res);
    out.write_all(format!("{}\n", res).as_bytes()).unwrap();
}
