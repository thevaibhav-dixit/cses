// CSES 1622: Creating Strings
// Category: Introductory Problems
// https://cses.fi/problemset/task/1622

use std::io::Write;

fn rec(chars: &[char], used: &mut [bool], prefix: &mut String, results: &mut Vec<String>) {
    if prefix.len() == chars.len() {
        results.push(prefix.clone());
        return;
    }
    for i in 0..chars.len() {
        if used[i] {
            continue;
        }
        // Equal letters may only be consumed left to right, so each distinct
        // letter is chosen at most once per position.
        if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
            continue;
        }
        used[i] = true;
        prefix.push(chars[i]);
        rec(chars, used, prefix, results);
        prefix.pop();
        used[i] = false;
    }
}

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    let s = sc.line();
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();

    let mut used = vec![false; chars.len()];
    let mut prefix = String::new();
    let mut results = Vec::new();
    rec(&chars, &mut used, &mut prefix, &mut results);

    writeln!(out, "{}", results.len()).unwrap();
    for r in &results {
        writeln!(out, "{}", r).unwrap();
    }
}
