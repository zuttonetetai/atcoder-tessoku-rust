use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u16,
    }
    for i in (0..10).rev() {
        let n1 = (n / 2u16.pow(i)) % 2;
        print!("{}", n1);
    }
}
