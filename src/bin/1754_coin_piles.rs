// CSES 1754: Coin Piles
// Category: Introductory Problems
// https://cses.fi/problemset/task/1754

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    let n = sc.next();
    for _ in 1..=n {
        let arr: Vec<i64> = sc.vec(2);
        let a = arr[0];
        let b = arr[1];
        if a == 0 && b == 0 {
            writeln!(out, "YES").unwrap();
            continue;
        }
        if (a + b) % 3 == 0 && a <= 2 * b && b <= 2 * a {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
