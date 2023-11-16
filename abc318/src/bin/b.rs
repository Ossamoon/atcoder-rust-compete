use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut sheet = vec![vec![0; 100]; 100];

    for (a, b, c, d) in abcd {
        for i in a..b {
            for j in c..d {
                sheet[i][j] = 1;
            }
        }
    }

    let sum = sheet
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>();

    println!("{}", sum);
}
