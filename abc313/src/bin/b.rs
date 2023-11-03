use proconio::input;

fn main() {
    input! (
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let x = solve(n, &ab);
    println!("{}", x);
}

fn solve(n: usize, ab: &[(usize, usize)]) -> i32 {
    let mut memo = vec![0; n]; // 0: unvisited, 1: maybe saikyo, 2: not saikyo

    for (a, b) in ab {
        let a = *a - 1;
        let b = *b - 1;
        if memo[a] == 0 {
            memo[a] = 1;
        }
        memo[b as usize] = -1;
    }

    let indices: Vec<usize> = memo
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x == 1)
        .map(|(index, _)| index)
        .collect();

    if indices.len() == 1 {
        return indices[0] as i32 + 1;
    } else {
        -1
    }
}
