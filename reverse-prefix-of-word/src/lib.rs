pub fn reverse_prefix(word: String, ch: char) -> String {
    let idx = word.find(ch);
    if let Some(i) = idx {
        let reversed: String = word[..=i].to_string().chars().rev().collect();
        println!("reversed={:?}", reversed);
        let result = format!("{}{}", reversed, &word[i + 1..]);
        result
    } else {
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "abcdefd".to_string();
        let ch = 'd';
        let output = "dcbaefd".to_string();
        let result = reverse_prefix(word, ch);
        assert_eq!(result, output);
    }
}
