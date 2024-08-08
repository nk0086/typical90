#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h]
    };

    let mut sum_row = vec![0; h];
    let mut sum_col = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_row[i] += a[i][j];
            sum_col[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", sum_row[i] + sum_col[j] - a[i][j]);
        }

        println!();
    }
}
