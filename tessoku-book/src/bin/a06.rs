use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q]
    }
    let mut add = 0;
    let ruisekiwa: Vec<usize> = a.iter().map(|x| {
        add += x;
        add
    }).collect();
    for i in 0..q {
        if lr[i].0 == 1 {
            println!("{}", ruisekiwa[lr[i].1 - 1]);
        } else {
            println!("{}", ruisekiwa[lr[i].1 - 1] - ruisekiwa[lr[i].0 - 2]);
        }

    }

}