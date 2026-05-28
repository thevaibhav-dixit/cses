// CSES 1069: Repetitions
// Category: Introductory Problems
// https://cses.fi/problemset/task/1069

use std::io::Write;

fn main() {
    let mut scanner = cses::stdin();
    let mut out = cses::stdout();

    let s = scanner.line();
    let bytes = s.as_bytes();
    let mut max_occ = 1usize;
    let mut curr = 1usize;
    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            curr += 1;
            if curr > max_occ {
                max_occ = curr;
            }
        } else {
            curr = 1;
        }
    }
    writeln!(out, "{max_occ}").unwrap();
}
