use proconio::input;

fn main() {
    input! {
        m: usize,
        s1: String,
        s2: String,
        s3: String,
    }

    let inf: usize = 1_000_000_000;
    let mut min_time = inf;

    for i in 0..m {
        for j in 0..m {
            for k in 0..m {
                let c1 = s1.chars().nth(i).unwrap();
                let c2 = s2.chars().nth(j).unwrap();
                let c3 = s3.chars().nth(k).unwrap();
                if !(c1 == c2 && c2 == c3) {
                    continue;
                }

                let time = if i == j && j == k {
                    2 * m + i
                } else if i == j || i == k {
                    m + i
                } else if j == k {
                    m + j
                } else {
                    std::cmp::max(i, std::cmp::max(j, k))
                };
                min_time = std::cmp::min(min_time, time);
            }
        }
    }

    if min_time == inf {
        println!("-1");
    } else {
        println!("{}", min_time);
    }
}
