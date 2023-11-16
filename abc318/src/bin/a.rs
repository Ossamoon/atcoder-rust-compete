use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        p: i32,
    }

    if n - m < 0 {
        println!("0");
        return;
    }

    let q = (n - m) / p;

    println!("{}", q + 1);
}
