#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let g = gcd(a, gcd(b, c));
    println!("{}", a / g + b / g + c / g - 3);
}

// gcd
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
