#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m]
    };

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ans = 0;
    for i in 0..n {
        let tmp = graph[i].iter().filter(|&&x| x < i).count();
        if tmp == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
