fn main() {
    let s = "hello world";

    let word = first_word(s);

    println!("First word of \"{}\" is \"{}\".", s, word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
