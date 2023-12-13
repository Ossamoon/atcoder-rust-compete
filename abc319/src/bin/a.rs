use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    };

    let hashmap = HashMap::from([
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ]);

    println!("{}", hashmap.get(s.as_str()).unwrap());
}
