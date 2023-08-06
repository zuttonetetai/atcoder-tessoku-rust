use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    input! {
        p: [usize; x],
        q: [usize; x],
    }
    for i in p.iter() {
        for j in q.iter() {
            if *i + *j == y {
                println!("Yes");
                return;
            }
        }

    }
    println!("No");
}
