fn main() {
    let s = String::from("I speak Goat Latin");
    println!("Hello, world!");
    println!("{}", to_goat_latin(s));
}
fn to_goat_latin(sentence: String) -> String {
    let mut result = String::new();

    for (i, word) in sentence.split_whitespace().enumerate() {
        let c = word.chars().next().unwrap();
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                result.push_str(&format!("{word}ma{a} ", word = word, a = "a".repeat(i + 1)))
            }
            _ => result.push_str(&format!(
                "{word_without_1st}{first_letter}ma{a} ",
                word_without_1st = &word[1..],
                first_letter = &word[0..1],
                a = "a".repeat(i + 1)
            )),
        }
    }
    result.trim_end().to_string()
}
