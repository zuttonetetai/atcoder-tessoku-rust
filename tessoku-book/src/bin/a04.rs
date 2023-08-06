use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    println!("{:0>1$b}", x, 10);
}
