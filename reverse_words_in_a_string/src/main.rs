fn main() {
    println!("Hello, world!");
    let mut input = "the sky is blue".to_string();
    let mut input2 = "    hello   world   ".to_string();
    println!("{}", reverse_words(input));
    println!("{}", reverse_words(input2));
}

pub fn reverse_words(s: String) -> String {
    let mut v = Vec::with_capacity(s.len());

    for word in s.split_whitespace() {
        v.push(word.to_string());
    }

    println!("DEBUGPRINT[1]: main.rs:10: v={:#?}", v);

    v.into_iter().rev().collect::<Vec<String>>().join(" ")
}
