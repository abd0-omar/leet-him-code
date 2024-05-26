mod lib;

fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    let s = "leetcode".to_string();
    let wordDict = vec!["leet".to_string(), "code".to_string()];
    // Output: true
    // Explanation: Return true because "leetcode" can be segmented as "leet code".
    // Example 2:
    //
    println!("{}", word_break(s, wordDict));
    let s = "applepenapple".to_string();
    let wordDict = vec!["apple".to_string(), "pen".to_string()];
    // Output: true
    // Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
    // Note that you are allowed to reuse a dictionary word.
    // Example 3:
    //
    // println!("{}", word_break(s, wordDict));
    // let s = "catsandog".to_string();
    // let wordDict = vec![
    //     "cats".to_string(),
    //     "dog".to_string(),
    //     "sand".to_string(),
    //     "and".to_string(),
    //     "cat".to_string(),
    // ];
    // println!("{}", word_break(s, wordDict));
    // Output: false
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    _word_break(s.as_bytes(), &word_dict, 0)
}

pub fn _word_break(s: &[u8], word_dict: &Vec<String>, st_idx: usize) -> bool {
    if st_idx == s.len() {
        return false;
    }

    let mut res = false;
    let mut word = String::new();
    for end in st_idx..s.len() {
        word.push(s[end] as char);
        println!("word={:?}", word);
        if word_dict.contains(&word) {
            res = true;
        }
        res = _word_break(s, word_dict, end + 1) || res;
    }
    res
}
