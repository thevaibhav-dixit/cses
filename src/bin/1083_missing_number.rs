// CSES 1083: Missing Number
// Category: Introductory Problems
// https://cses.fi/problemset/task/1083
//

use std::io::Write;

fn main() {
    let mut scanner = cses::stdin();
    let mut out = cses::stdout();

    let n = scanner.next::<usize>();
    let input_arr = scanner.vec::<usize>(n - 1);

    let sum_of_n_numbers = n * (n + 1) / 2;
    let sum_of_input_numbers: usize = input_arr.iter().sum();

    let missing_number = sum_of_n_numbers - sum_of_input_numbers;
    writeln!(out, "{missing_number}").unwrap();
}
