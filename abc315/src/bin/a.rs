use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! { s:  Chars  };

    println!(
        "{}",
        &s.iter()
            .filter(|&c| *c != 'a' && *c != 'e' && *c != 'i' && *c != 'o' && *c != 'u')
            .collect::<String>()
    );
}
