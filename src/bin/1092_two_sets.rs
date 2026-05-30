// CSES 1092: Two Sets
// Category: Introductory Problems
// https://cses.fi/problemset/task/1092

use std::io::Write;

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();

    let n: i64 = sc.next();

    let total_sum = n * (n + 1) / 2;
    if total_sum % 2 != 0 {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    if n % 2 == 0 {
        let mut set1 = Vec::new();
        let mut set2 = Vec::new();
        for i in (1..=n).step_by(4) {
            set1.push(i);
            set1.push(n - i + 1);
            set2.push(i + 1);
            set2.push(n - i);
        }
        writeln!(out, "{}", n / 2).unwrap();
        writeln!(
            out,
            "{}",
            set1.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
        writeln!(out, "{}", n / 2).unwrap();
        writeln!(
            out,
            "{}",
            set2.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    } else {
        let mut skip_value = 0;
        let mut set1 = Vec::new();
        let mut set2 = Vec::new();
        let mut iterate_till = 0;
        let mut total = 0;
        for i in (1..=n).rev() {
            if total + i < total_sum / 2 {
                set1.push(i);
                total += i;
                iterate_till = i;
            } else {
                set1.push(total_sum / 2 - total);
                skip_value = total_sum / 2 - total;
                break;
            }
        }
        if iterate_till == 0 {
            iterate_till = n;
        }
        for i in 1..iterate_till {
            if i != skip_value {
                set2.push(i);
            }
        }

        writeln!(out, "{}", set1.len()).unwrap();
        writeln!(
            out,
            "{}",
            set1.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
        writeln!(out, "{}", set2.len()).unwrap();
        writeln!(
            out,
            "{}",
            set2.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    }
}
