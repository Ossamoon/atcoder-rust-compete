use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let s: String = (0..=n)
        .map(|i| {
            for j in (1..=9).filter(|j| n % j == 0) {
                if i % (n / j) == 0 {
                    return (j as u8 + '0' as u8) as char;
                }
            }
            return '-';
        })
        .collect::<String>();

    println!("{}", s);
}
