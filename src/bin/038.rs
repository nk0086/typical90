#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        a: i128, b: i128
    };

    let ans = lcm(a, b);
    println!(
        "{}",
        if ans > 10_i128.pow(18) {
            "Large".to_string()
        } else {
            ans.to_string()
        }
    );
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}
