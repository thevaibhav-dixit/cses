// CSES 2205: Gray Code
// Category: Introductory Problems
// https://cses.fi/problemset/task/2205

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: u32 = sc.next();
    let size = 1 << n;

    for i in 0..size {
        let gray_code = i ^ (i >> 1);
        writeln!(out, "{:0width$b}", gray_code, width = n as usize).unwrap();
    }
}
