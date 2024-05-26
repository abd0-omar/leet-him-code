pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<&_> = word_dict.iter().collect();
    let mut memo = HashMap::new();
    word_break_helper(&s, 0, &word_set, &mut memo)
}

use std::collections::{HashMap, HashSet};
fn word_break_helper(
    s: &str,
    start: usize,
    word_set: &HashSet<&String>,
    memo: &mut HashMap<usize, bool>,
) -> bool {
    if start == s.len() {
        return true;
    }
    if let Some(&result) = memo.get(&start) {
        return result;
    }

    for end in start..s.len() {
        let cur_word = &s[start..=end].to_string();
        if word_set.contains(cur_word) {
            let result = word_break_helper(s, end + 1, word_set, memo);
            if result {
                memo.insert(start, result);
                return result;
            }
        }
    }

    memo.insert(start, false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let s = "catsanddog".to_string();
        let word_dict = vec![
            "cat".to_string(),
            "cats".to_string(),
            "and".to_string(),
            "sand".to_string(),
            "dog".to_string(),
        ];
        assert!(word_break(s.clone(), word_dict.clone()));

        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert!(word_break(s.clone(), word_dict.clone()));

        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];
        assert!(!word_break(s.clone(), word_dict.clone()));
    }
}
