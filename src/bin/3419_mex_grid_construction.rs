// CSES 3419: Mex Grid Construction
// Category: Introductory Problems
// https://cses.fi/problemset/task/3419

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: usize = sc.next();
    for i in 0..n {
        let row = (0..n)
            .map(|j| (i ^ j).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(out, "{}", row).unwrap();
    }
}
