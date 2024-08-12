#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize
    };

    let ans = solve(h, w);
    println!("{}", ans);
}

fn solve(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        return h * w;
    }

    let a = (h + 1) / 2;
    let b = (w + 1) / 2;

    return a * b;
}
