#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
        b: [i64; n],
    };

    let sum = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i64>();

    println!(
        "{}",
        if sum <= k && (k - sum) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
