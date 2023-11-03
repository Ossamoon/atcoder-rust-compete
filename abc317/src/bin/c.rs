use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        streets: [(usize, usize, i64); m]
    }
    let mut graph = vec![vec![-1; n]; n];
    for street in streets {
        let town1 = street.0;
        let town2 = street.1;
        let cost = street.2;
        graph[town1 - 1][town2 - 1] = cost;
        graph[town2 - 1][town1 - 1] = cost;
    }

    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, dfs(&graph, &vec![i], &n));
    }

    println!("{}", ans);
}

fn dfs(graph: &Vec<Vec<i64>>, path: &Vec<usize>, n: &usize) -> i64 {
    let mut max = 0;
    let prev = path[path.len() - 1];
    for i in 0..*n {
        if path.contains(&i) || graph[prev][i] == -1 {
            continue;
        }
        let mut new_path = path.clone();
        new_path.push(i);
        max = std::cmp::max(max, dfs(graph, &new_path, n) + graph[prev][i]);
    }

    max
}
