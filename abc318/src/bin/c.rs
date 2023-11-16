use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: u64,
        mut f: [u64; n],
    }

    f.sort();
    f.reverse();
    let mut sum = 0;

    for i in 0..(n / d) {
        let slice = &f[(i * d)..((i + 1) * d)];
        sum += min(slice.iter().sum::<u64>(), p);
    }

    let last_slice = &f[(n / d) * d..];
    sum += min(last_slice.iter().sum::<u64>(), p);

    println!("{}", sum);
}

// 他の人の回答で美しかったもの
//
// fn main() {
//     input! {
//         n: usize,
//         d: usize,
//         p: usize,
//         mut f: [usize; n],
//     }
//     f.sort();
//     f.reverse();
//     let ans = f
//         .chunks(d)
//         .map(|x| x.iter().sum::<usize>())
//         .map(|x| x.min(p))
//         .sum::<usize>();
//     println!("{}", ans);
// }
