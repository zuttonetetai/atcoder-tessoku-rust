use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    for i in 0..n {
        for j in 0..n {
            if j == i { continue; }
            for k in 0..n {
                if k == i || k == j { continue; }
                if a[i] + a[j] + a[k] == 1000 {
                    println!("Yes");
                    return;
                };
            }
        }
    }
    println!("No");
}
