// CSES 1068: Weird Algorithm
// Category: Introductory Problems
// https://cses.fi/problemset/task/1068

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    let mut n: u64 = sc.next();
    write!(out, "{n}").unwrap();
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        write!(out, " {n}").unwrap();
    }
    writeln!(out).unwrap();
}
