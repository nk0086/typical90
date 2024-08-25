#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, c: i64,
    };

    if a < c.pow(b as u32) {
        println!("Yes");
    } else {
        println!("No");
    }
}
