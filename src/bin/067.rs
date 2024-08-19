#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1},
};

#[fastout]
fn main() {
    input! {
        s: String
    };

    let (n, k) = {
        let mut iter = s.split_whitespace();
        let n = iter.next().unwrap().parse::<String>().unwrap();
        let k = iter.next().unwrap().parse::<u32>().unwrap();
        (n, k)
    };
    let ans = n_to_k(n, k);
    println!("{}", ans);
}

fn n_to_k(n: String, k: u32) -> String {
    let n = n.to_string();
    let mut ans = 0;

    for (i, c) in n.chars().enumerate() {
        ans += c.to_digit(10).unwrap() * k.pow((n.len() - i - 1) as u32);
    }

    // 10進数をk進数に変換
    let mut res = vec![];
    while ans > 0 {
        res.push(((ans % k) as u8 + '0' as u8) as char);
        ans /= k;
    }

    res.iter().collect::<String>()
}
