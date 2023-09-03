use proconio::input;

// 答えは10^9秒を超えないこととする。
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut t_min: usize = 1;
    let mut t_max = 10usize.pow(9);
    while t_min < t_max {
        let mid = (t_min + t_max) / 2;
        let mut print_num = 0;
        for i in 0..n {
            print_num += mid / a[i];
        }
        if print_num < k {
            t_min = mid+1 ;
        } else {
            t_max = mid;
        }
        //println!("{} {} {}", t_max, t_min, print_num);
    }
    println!("{}", t_max);
}
