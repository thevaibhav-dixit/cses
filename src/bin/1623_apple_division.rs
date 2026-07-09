// CSES 1623: Apple Division
// Category: Introductory Problems
// https://cses.fi/problemset/task/1623

use std::io::Write;

fn recurse(arr: &Vec<i64>, index: usize, curr_sum: i64, total_sum: i64, res: &mut i64) {
    if index == arr.len() {
        let other_sum = total_sum - curr_sum;
        let curr_diff = (curr_sum - other_sum).abs();
        if curr_diff < *res {
            *res = curr_diff;
        }
        return;
    }

    //take the curr element
    recurse(arr, index + 1, curr_sum + arr[index], total_sum, res);

    // do not take the curr element
    recurse(arr, index + 1, curr_sum, total_sum, res);
}

fn main() {
    let mut sc = cses::stdin();
    let mut out = cses::stdout();
    let n = sc.next();
    let arr: Vec<i64> = sc.vec(n);
    let sum = arr.iter().sum::<i64>();
    let mut res = i64::MAX;

    // use take/not_take strategy
    recurse(&arr, 0, 0, sum, &mut res);

    out.write_all(format!("{}\n", res).as_bytes()).unwrap();
}
