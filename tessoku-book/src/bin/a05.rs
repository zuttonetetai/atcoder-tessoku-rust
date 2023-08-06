use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut count= 0;
    for x in 1..=n {
        for y in 1..=n {
            let z = k - x - y;
            if z > 0 && z <= n {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
