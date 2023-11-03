use proconio::input;

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n]
    }
    let (max_i, max_fs) = fs.iter().enumerate().max_by_key(|&(_, &(_, s))| s).unwrap();

    let mut max_point = 0;
    for i in 0..n {
        if i == max_i {
            continue;
        }
        let point = if fs[i].0 == max_fs.0 {
            fs[i].1 / 2
        } else {
            fs[i].1
        };
        max_point = std::cmp::max(max_point, point);
    }
    println!("{}", max_fs.1 + max_point);
}
