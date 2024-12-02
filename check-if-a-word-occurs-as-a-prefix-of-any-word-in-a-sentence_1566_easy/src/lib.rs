// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
#[allow(dead_code)]
struct Solution;

#[derive(Clone)]
struct Trie {
    child: Vec<Option<Trie>>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            child: vec![None; 26],
            is_leaf: false,
        }
    }

    fn add(&mut self, word: &[u8]) {
        let mut trie = self;
        for letter in word {
            let letter_idx = (letter - b'a') as usize;
            if trie.child[letter_idx].is_none() {
                trie.child[letter_idx] = Some(Trie::new());
            }
            trie = trie.child[letter_idx].as_mut().unwrap();
        }
        trie.is_leaf = true;
    }

    fn is_prefix(&self, word: &[u8]) -> bool {
        let mut trie = self;
        for letter in word {
            let letter_idx = (letter - b'a') as usize;
            if trie.child[letter_idx].is_none() {
                return false;
            }
            trie = trie.child[letter_idx].as_ref().unwrap();
            if trie.is_leaf {
                return true;
            }
        }
        false
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        // trivial O(n^2) solution
        // but would like to complicate it more
        let mut trie = Trie::new();
        trie.add(search_word.as_bytes());
        for (idx, word) in sentence.split_whitespace().enumerate() {
            if trie.is_prefix(word.as_bytes()) {
                return idx as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let sentence = "i love eating burger".to_string();
        let search_word = "burger".to_string();
        let output = 4;
        let result = Solution::is_prefix_of_word(sentence, search_word);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let sentence = "this problem is an easy problem".to_string();
        let search_word = "pro".to_string();
        let output = 2;
        let result = Solution::is_prefix_of_word(sentence, search_word);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let sentence = "i am tired".to_string();
        let search_word = "you".to_string();
        let output = -1;
        let result = Solution::is_prefix_of_word(sentence, search_word);
        assert_eq!(result, output);
    }
}
