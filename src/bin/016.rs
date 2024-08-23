#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1},
};

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64, b: i64, c: i64,
    };

    let mut ans = 10000;
    for i in 0..10000 {
        for j in 0..(10000 - i) {
            let tmp = n - a * i - b * j;
            if tmp >= 0 && tmp % c == 0 {
                ans = ans.min(i + j + tmp / c);
            }
        }
    }

    println!("{}", ans);
}
