#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    let mut map = HashMap::new();
    for (i, si) in s.iter().enumerate() {
        if !map.contains_key(si) {
            map.insert(si, 1);
            println!("{}", i + 1);
        }
    }
}
