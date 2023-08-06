use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    input! {
        n: [usize; x],
    }

    for i in 0..x {
        if n[i] == y {
            println!("Yes");
            return;
        }
    }
    println!("No");

}
