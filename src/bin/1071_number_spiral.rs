// CSES 1071: Number Spiral
// Category: Introductory Problems
// https://cses.fi/problemset/task/1071

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n = sc.next();

    for _ in 0..n {
        let co_ordinates: Vec<i64> = sc.vec(2);
        let x = co_ordinates[0];
        let y = co_ordinates[1];
        let max = std::cmp::max(x, y);
        let mut ans = 0;
        if max % 2 == 0 {
            ans = max * max - max + 1 + (x - y);
        } else {
            ans = max * max - max + 1 - (x - y);
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
