// CSES 1070: Permutations
// Category: Introductory Problems
// https://cses.fi/problemset/task/1070
//
use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: i32 = sc.next();

    if n == 1 {
        writeln!(out, "1").unwrap();
    } else if n <= 3 {
        writeln!(out, "NO SOLUTION").unwrap();
    } else if n == 4 {
        writeln!(out, "2 4 1 3").unwrap();
    } else {
        let mut ans = Vec::new();

        for i in (1..=n).step_by(2) {
            ans.push(i);
        }
        for i in (2..=n).step_by(2) {
            ans.push(i);
        }

        for i in ans {
            write!(out, "{} ", i).unwrap();
        }
        writeln!(out).unwrap();
    }
}
