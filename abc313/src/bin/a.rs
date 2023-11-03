use proconio::input;

fn main() {
    input! {
        n: i32,
        p: [i32; n],
    }

    let x = solve(&p);

    println!("{}", x);
}

fn solve(p: &[i32]) -> i32 {
    if p.len() == 1 {
        return 0;
    }

    let max = *p[1..].iter().max().unwrap();

    if p[0] > max {
        0
    } else {
        max - p[0] + 1
    }
}
