// CSES 1094: Increasing Array
// Category: Introductory Problems
// https://cses.fi/problemset/task/1094

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    let n: usize = sc.next();
    let mut arr: Vec<u64> = sc.vec(n);

    let mut res = 0;

    for i in 1..n {
        if arr[i] < arr[i - 1] {
            res += arr[i - 1] - arr[i];
            arr[i] = arr[i - 1];
        }
    }

    writeln!(out, "{res}").unwrap();
}
