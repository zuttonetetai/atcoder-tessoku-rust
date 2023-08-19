use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut res = vec![vec![0; w+1]; h+1];
    for i in abcd.iter() {
        let x1 = i.0 - 1;
        let y1 = i.1 - 1;
        let x2 = i.2 - 1;
        let y2 = i.3 - 1;

        res[x1][y1] += 1;
        res[x1][y2 + 1] -= 1;
        res[x2+1][y1] -= 1;
        res[x2+1][y2+1] += 1;
    }

    for i in 0..h {
        let mut add = 0;
        for j in 1..=w {
            add = res[i][j-1];
            res[i][j] += add;
        }
    }

    for i in 0..w {
        let mut add = 0;
        for j in 1..=h {
            add = res[j-1][i];
            res[j][i] += add;
        }
    }
    for i in 0..h {
        let res_srt = res[i][..res[i].len() - 1].iter().map(|x| x.to_string()).collect::<Vec<String>>();
        println!("{:}", res_srt.join(" "));
    }
}
