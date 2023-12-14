use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut max_len = 1;

    for i in 0..s.len() {
        for j in (i + 1)..s.len() {
            let slice = &s[i..=j];
            let rev = slice.chars().rev().collect::<String>();
            if slice == rev {
                max_len = std::cmp::max(max_len, slice.len());
            }
        }
    }

    println!("{}", max_len);
}
