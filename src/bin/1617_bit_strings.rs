// CSES 1617: Bit Strings
// Category: Introductory Problems
// https://cses.fi/problemset/task/1617

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    const MOD: u64 = 1_000_000_007;
    let n: i64 = sc.next();
    let ans = (1..=n).fold(1, |acc, _| (acc * 2) % MOD);
    writeln!(out, "{}", ans).unwrap();
}
