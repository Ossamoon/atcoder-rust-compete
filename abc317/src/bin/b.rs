use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut memo = vec![0; 1000];
    for an in a {
        memo[an - 1] = 1;
    }

    let mut prev = 0;
    for (i, &m) in memo.iter().enumerate() {
        if m == 0 && prev == 1 {
            println!("{}", i + 1);
            return;
        }
        prev = m;
    }

    println!("{}", 1000 - n);
}
