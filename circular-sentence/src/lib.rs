pub fn is_circular_sentence(sentence: String) -> bool {
    let parts: Vec<&[u8]> = sentence
        .split_whitespace()
        .map(|word| word.as_bytes())
        .collect();

    if parts.first().unwrap().first().unwrap() != parts.last().unwrap().last().unwrap() {
        return false;
    }

    for i in 1..parts.len() {
        if parts[i].first().unwrap() != parts[i - 1].last().unwrap() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sentence = "leetcode exercises sound delightful".to_string();
        let output = true;
        let result = is_circular_sentence(sentence);
        assert_eq!(result, output);
    }
}
