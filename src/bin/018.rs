#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        t: usize,
        l: i64, x: i64, y: i64,
        q: usize,
        e:  [i64; q],
    };

    for ei in e {
        let ans = calc(l, x, y, ei);
        println!("{}", ans);
    }
}

fn calc(l: i64, x: i64, y: i64, e: i64) -> f64 {
    todo!();
}
