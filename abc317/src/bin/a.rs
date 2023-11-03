use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i32,
        x: i32,
        ps: [i32; n]
    }

    let min = x - h;

    for i in 0..n {
        if ps[i] >= min {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", ps[n - 1]);
}
