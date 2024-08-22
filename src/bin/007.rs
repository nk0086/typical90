#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        m: [i64; q],
    };

    a.sort();

    for mi in m {
        let i = check(&a, mi);
        let mut ans = (mi - a[i]).abs();
        if i + 1 < n {
            ans = ans.min((mi - a[i + 1]).abs());
        }

        println!("{}", ans);
    }
}

fn check(a: &[i64], m: i64) -> usize {
    let mut ok = 0;
    let mut ng = a.len();

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if a[mid] < m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    return ok;
}
