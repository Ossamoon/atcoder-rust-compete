use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: [usize; 9],
    };

    let c = c.iter().map(|i| i - 1).collect::<Vec<_>>();

    let lines = vec![
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    let mut check_lines = vec![];

    for (i, j, k) in lines {
        if c[i] == c[j] {
            check_lines.push(((i, j), k));
        } else if c[j] == c[k] {
            check_lines.push(((j, k), i));
        } else if c[k] == c[i] {
            check_lines.push(((k, i), j));
        }
    }

    let mut count: u32 = 0;

    'order: for order in (0..9).permutations(9) {
        for ((a1, a2), b) in check_lines.iter() {
            if order[*a1] < order[*b] && order[*a2] < order[*b] {
                continue 'order;
            }
        }
        count += 1;
    }

    println!("{}", (count as f64) / (1..=9).product::<u32>() as f64);
}
