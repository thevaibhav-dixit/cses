// CSES 1755: Palindrome Reorder
// Category: Introductory Problems
// https://cses.fi/problemset/task/1755

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let s = sc.line();
    let n = s.len();

    let mut counts = std::collections::HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0u64) += 1;
    }

    // A palindrome allows at most one character with an odd count (the middle).
    let mut odd_char = None;
    for (&c, &v) in &counts {
        if v % 2 == 1 {
            if odd_char.is_some() {
                writeln!(out, "NO SOLUTION").unwrap();
                return;
            }
            odd_char = Some(c);
        }
    }

    let mut ans = vec!['\0'; n];
    let mut i = 0;
    for (&c, &v) in &counts {
        for _ in 0..v / 2 {
            ans[i] = c;
            ans[n - 1 - i] = c;
            i += 1;
        }
    }
    if let Some(c) = odd_char {
        ans[n / 2] = c;
    }

    writeln!(out, "{}", ans.iter().collect::<String>()).unwrap();
}
