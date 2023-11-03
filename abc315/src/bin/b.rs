use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m]
    }
    let sum = d.iter().sum::<usize>();
    let mid = (sum + 1) / 2;
    let mut temp = 0;
    for i in 0..m {
        if temp + d[i] >= mid {
            println!("{} {}", i + 1, mid - temp);
            break;
        }
        temp += d[i];
    }
}
