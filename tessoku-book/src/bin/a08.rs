use proconio::{input, fastout};

//縦がx軸、横がy軸
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[u32; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut ruisekiwa = vec![vec![0; w+1]; h+1];
    let mut add = 0;
    // 横方向に累積和
    for i in 1..=h {
        add = 0;
        for ii in 1..=w {
            add += x[i-1][ii-1];
            ruisekiwa[i][ii] += add;
        }
    }
    // 縦方向に累積和
    for i in 0..=w {
        for ii in 0..h {
            add = ruisekiwa[ii][i];
            ruisekiwa[ii+1][i] += add;
        }
    }
    for i in 0..q {
        let (x1, y1, x2, y2) = abcd[i];
        let res = ruisekiwa[x2][y2] + ruisekiwa[x1 - 1][y1 - 1] - ruisekiwa[x1-1][y2] - ruisekiwa[x2][y1-1];

        println!("{}", res);
    }

}
