// CSES 3399: Raab Game I
// Category: Introductory Problems
// https://cses.fi/problemset/task/3399

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: usize = sc.next();
        let b: usize = sc.next();

        let possible = (a == 0 && b == 0) || (a >= 1 && b >= 1 && a + b <= n);
        if !possible {
            writeln!(out, "NO").unwrap();
            continue;
        }

        let k = a + b;
        let p1: Vec<usize> = (1..=n).collect();
        let mut p2: Vec<usize> = (0..k).map(|i| (i + a) % k + 1).collect();
        p2.extend(k + 1..=n);

        writeln!(out, "YES").unwrap();
        let fmt = |v: &[usize]| {
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };
        writeln!(out, "{}", fmt(&p1)).unwrap();
        writeln!(out, "{}", fmt(&p2)).unwrap();
    }
}
