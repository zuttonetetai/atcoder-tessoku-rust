use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut res = vec![0; d + 1];
    for i in 0..n {
        res[lr[i].0 - 1] += 1;
        res[lr[i].1] -= 1;
    }
    let mut add = 0;
    for i in 0..d {
        add += res[i];
        println!("{}", add);
    }
}
