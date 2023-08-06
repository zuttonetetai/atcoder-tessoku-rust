use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: usize,
        b: usize,
    }
    for i in a..=b {
        if 100  %  i == 0 {
            println!("Yes");
            return Ok(());
        }
    }
    println!("No");
    Ok(())
}
