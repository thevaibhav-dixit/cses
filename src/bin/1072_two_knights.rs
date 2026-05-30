// CSES 1072: Two Knights
// Category: Introductory Problems
// https://cses.fi/problemset/task/1072

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: i64 = sc.next();

    for i in 1..=n {
        let total_pos = i * i * (i * i - 1) / 2;
        let attack_pos = 4 * (i - 1) * (i - 2);
        writeln!(out, "{}", total_pos - attack_pos).unwrap();
    }
}
