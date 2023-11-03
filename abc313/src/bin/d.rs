use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
        k: usize,
    }

    let mut a = vec![0; n]; // 答えを格納する配列
    let mut m = vec![0; n]; // a[i] - a[0] が奇数かどうか
    let mut x = (1..=k).collect::<Vec<_>>(); // printする配列

    println!("? {}", &x.iter().join(" "));
    stdout().flush().unwrap();
    input! {
        t: i32,
    }
    let memo = t; // a[0] + ... + a[k - 1] の和が奇数かどうか
    let mut memo2 = -1; // a[1] + ... + a[k] の和が奇数かどうか

    for i in k..n {
        x = (2..=k).collect();
        x.push(i + 1);
        println!("? {}", &x.iter().join(" "));
        stdout().flush().unwrap();
        input! {
            t: i32,
        }
        m[i] = (t - memo + 2) % 2;

        if i == k {
            memo2 = t;
        }
    }

    if memo2 == -1 {
        std::process::exit(1);
    }

    for i in 1..k {
        x = (1..=i).chain(i + 2..=k + 1).collect::<Vec<_>>();
        println!("? {}", &x.iter().join(" "));
        stdout().flush().unwrap();
        input! {
            t: i32,
        }
        m[i] = (memo2 - t + 2) % 2;
    }

    if m[0..k].iter().sum::<i32>() % 2 == memo {
        // a[0..k]でa[0]と異なる要素の個数が偶数個 => memo == 0 => a[0..k]で1が偶数個 => a[0] = 0
        // a[0..k]でa[0]と異なる要素の個数が奇数個 => memo == 1 => a[0..k]で1が奇数個 => a[0] = 0
        for i in 0..n {
            if m[i] == 0 {
                a[i] = 0;
            } else {
                a[i] = 1;
            }
        }
    } else {
        // a[0] = 1
        for i in 0..n {
            if m[i] == 0 {
                a[i] = 1;
            } else {
                a[i] = 0;
            }
        }
    }

    println!("! {}", &a[0..n].iter().join(" "));
    stdout().flush().unwrap();
}
