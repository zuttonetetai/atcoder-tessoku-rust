use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    for i in 0..d {
        let aa = &a[..lr[i].0 -1];
        let bb = &a[lr[i].1..];
        let aamax = aa.iter().max().unwrap();
        let bbmax = bb.iter().max().unwrap();
        println!("{:?}", aamax.max(bbmax));
    }
}
