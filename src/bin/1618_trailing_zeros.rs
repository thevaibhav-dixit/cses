// CSES 1618: Trailing Zeros
// Category: Introductory Problems
// https://cses.fi/problemset/task/1618

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: i64 = sc.next();
    // legendres formula: n! has n/5 + n/25 + n/125 + ... trailing zeros
    let mut curr_power_of_5 = 5;
    let mut ans = 0;
    while curr_power_of_5 <= n {
        ans += n / curr_power_of_5;
        curr_power_of_5 *= 5;
    }
    writeln!(out, "{}", ans).unwrap();
}
