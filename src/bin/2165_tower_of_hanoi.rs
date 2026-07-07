// CSES 2165: Tower of Hanoi
// Category: Introductory Problems
// https://cses.fi/problemset/task/2165

use std::io::Write;
fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: u32 = sc.next();
    writeln!(out, "{}", (1 << n) - 1).unwrap(); // Very elegant derivation

    fn hanoi(n: u32, from: char, to: char, aux: char, out: &mut impl Write) {
        if n == 0 {
            return;
        }
        hanoi(n - 1, from, aux, to, out);
        writeln!(out, "{} {}", from, to).unwrap();
        hanoi(n - 1, aux, to, from, out);
    }

    hanoi(n, '1', '3', '2', &mut out);
}
