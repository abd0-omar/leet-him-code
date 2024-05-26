pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut total_subsets = Vec::new();
    let word_set: HashSet<_> = word_dict.iter().collect();
    subsets(&s, 0, String::new(), &mut total_subsets, &word_set);
    total_subsets
}

use std::collections::HashSet;
pub fn subsets(
    s: &str,
    start: usize,
    mut cur_sentence: String,
    result: &mut Vec<String>,
    word_set: &HashSet<&String>,
) {
    if start == s.len() {
        result.push(cur_sentence);
        println!("result={:?}", result);
        return;
    }

    for end in start..s.len() {
        let cur_word = &s[start..=end];
        if word_set.contains(&cur_word.to_string()) {
            let original_sentence = cur_sentence.clone();
            if !cur_sentence.is_empty() {
                cur_sentence.push(' ');
            }
            cur_sentence.push_str(cur_word);
            subsets(s, end + 1, cur_sentence, result, word_set);
            cur_sentence = original_sentence;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "catsanddog".to_string();
        let word_dict = vec![
            "cat".to_string(),
            "cats".to_string(),
            "and".to_string(),
            "sand".to_string(),
            "dog".to_string(),
        ];
        let mut output = vec!["cats and dog".to_string(), "cat sand dog".to_string()];
        output.sort_unstable();
        let mut result = word_break(s, word_dict);
        result.sort_unstable();
        assert_eq!(result, output);
    }
}
