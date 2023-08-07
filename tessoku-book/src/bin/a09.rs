use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut res: Vec<Vec<u16>> = vec![vec![0; w]; h];
    for i in 0..n {
        let (x1, y1, x2, y2) = abcd[i];
        for ii in x1-1..x2 {
            for jj in y1-1..y2 {
                res[ii][jj] += 1;
            }



            //for jj in y1-1..y2 {
            //    res[ii][jj] += 1;
            //}
        }
    }
    res.iter().for_each(|s| {
        //let a = s.iter().map(|ss| ss.to_string()).collect::<Vec<String>>();
        //fmt::Formatter::write_str(&mut self, data)

        //println!("{:}", &s[..].join(" "));
    });
    //for i in 0..h {
    //    println!("{:}", res_str[i].join(" "));
    //}
    //println!("{:?}", res);
}
