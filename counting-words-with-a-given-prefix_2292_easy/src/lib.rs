// https://leetcode.com/problems/counting-words-with-a-given-prefix/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        // brute-force is straight forward, and the input is low
        // but I'll do trie anyway, and trie is straight forward too
        let mut trie = Trie::new();
        for word in words {
            trie.insert_word(word.as_bytes());
        }
        trie.count_prefix(pref.as_bytes())
    }
}

struct Trie {
    // only lowercase English letters
    child: [Option<Box<Trie>>; 26],
    count: i32,
}

impl Trie {
    fn new() -> Self {
        Self {
            child: [const { None }; 26],
            count: 0,
        }
    }

    fn insert_word(&mut self, word: &[u8]) {
        let mut trie = self;
        for &letter in word {
            let letter_idx = (letter - b'a') as usize;
            if trie.child[letter_idx].is_none() {
                trie.child[letter_idx] = Some(Box::new(Trie::new()));
            }
            trie = trie.child[letter_idx].as_mut().unwrap();
            trie.count += 1;
            // dbg!(&trie);
        }
    }

    fn count_prefix(&self, word: &[u8]) -> i32 {
        let mut trie = self;
        for &letter in word {
            let letter_idx = (letter - b'a') as usize;
            if trie.child[letter_idx].is_none() {
                return 0;
            }
            trie = trie.child[letter_idx].as_ref().unwrap();
        }
        trie.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec![
            "pay".to_string(),
            "attention".to_string(),
            "practice".to_string(),
            "attend".to_string(),
        ];
        let pref = "at".to_string();
        let output = 2;
        let result = Solution::prefix_count(words, pref);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec![
            "leetcode".to_string(),
            "win".to_string(),
            "loops".to_string(),
            "success".to_string(),
        ];
        let pref = "code".to_string();
        let output = 0;
        let result = Solution::prefix_count(words, pref);
        assert_eq!(result, output);
    }
}
