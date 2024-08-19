#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize, p: u128, q: u128,
        a: [u128; n],
    };

    let mut ans = 0;
    for comb in (0..n).combinations(5) {
        let tmp = a[comb[0]] * a[comb[1]] % p * a[comb[2]] % p * a[comb[3]] % p * a[comb[4]] % p;
        if tmp == q {
            ans += 1;
        }
    }

    println!("{}", ans);
}
