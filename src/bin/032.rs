#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    };

    let xy = xy.into_iter().collect::<std::collections::HashSet<_>>();
    let mut ans = std::i64::MAX;
    for comb in (0..n).permutations(n) {
        ans = ans.min(solve(comb, &a, &xy));
    }

    println!("{}", if ans == std::i64::MAX { -1 } else { ans });
}

fn solve(
    comb: Vec<usize>,
    a: &Vec<Vec<i64>>,
    xy: &std::collections::HashSet<(usize, usize)>,
) -> i64 {
    let mut res = 0;
    for i in 0..comb.len() {
        if i < comb.len() - 1 && xy.contains(&(comb[i], comb[i + 1])) {
            return std::i64::MAX;
        }

        res += a[comb[i]][i];
    }

    res
}
