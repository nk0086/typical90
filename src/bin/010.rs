#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(i64, i64); n],
        q: usize,
        lr: [(i64, i64); q],
    };

    let mut sum_1 = vec![0; n + 1];
    let mut sum_2 = vec![0; n + 1];

    for i in 0..n {
        sum_1[i + 1] = sum_1[i];
        sum_2[i + 1] = sum_2[i];
        if cp[i].0 == 1 {
            sum_1[i + 1] += cp[i].1;
        } else {
            sum_2[i + 1] += cp[i].1;
        }
    }

    for (l, r) in lr {
        let ans_1 = sum_1[r as usize] - sum_1[l as usize - 1];
        let ans_2 = sum_2[r as usize] - sum_2[l as usize - 1];
        println!("{} {}", ans_1, ans_2);
    }
}
