// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        // there is a trie solution not by doing two tries (prefix trie, and
        // every suffix trie)
        // but with doing a weird prefix suffix trie idk,
        // it's available in
        // https://leetcode.com/problems/count-prefix-and-suffix-pairs-ii/
        // but don't recommend it

        let n = words.len();
        let mut result = 0;
        for i in 0..n - 1 {
            let cur_word = words[i].as_bytes();
            for j in i + 1..n {
                let other_word = words[j].as_bytes();
                if other_word.starts_with(cur_word) && other_word.ends_with(cur_word) {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec![
            "a".to_string(),
            "aba".to_string(),
            "ababa".to_string(),
            "aa".to_string(),
        ];
        let output = 4;
        let result = Solution::count_prefix_suffix_pairs(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec![
            "pa".to_string(),
            "papa".to_string(),
            "ma".to_string(),
            "mama".to_string(),
        ];
        let output = 2;
        let result = Solution::count_prefix_suffix_pairs(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec!["abab".to_string(), "ab".to_string()];
        let output = 0;
        let result = Solution::count_prefix_suffix_pairs(words);
        assert_eq!(result, output);
    }
}
