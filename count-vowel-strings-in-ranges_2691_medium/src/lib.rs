// https://leetcode.com/problems/count-vowel-strings-in-ranges/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = words.len();
        // make `words_filter` filters "vowel strings"
        let words_score: Vec<i32> = words
            .into_iter()
            .map(|word| {
                if is_vowel_string(word.as_bytes()) {
                    1
                } else {
                    0
                }
            })
            .collect();
        let mut prefix_sum = vec![0; n + 1];
        for i in 1..n + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + words_score[i - 1];
        }
        // dbg!(&prefix_sum);
        // running 1 test
        // [src/lib.rs:24:9] &prefix_sum = [
        //             0,
        //             1,
        //             1,
        //             2,
        //             3,
        //             4,
        //     ]
        // let words = vec![
        // "aba".to_string(),
        // "bcb".to_string(),
        // "ece".to_string(),
        // "aa".to_string(),
        // "e".to_string(),
        // ];
        // query 1, 2
        // result => 1
        //
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            result.push(prefix_sum[to + 1] - prefix_sum[from]);
        }
        result
    }
}

fn is_vowel_string(word: &[u8]) -> bool {
    if (word[0] == b'a' || word[0] == b'e' || word[0] == b'i' || word[0] == b'o' || word[0] == b'u')
        && (word[word.len() - 1] == b'a'
            || word[word.len() - 1] == b'e'
            || word[word.len() - 1] == b'i'
            || word[word.len() - 1] == b'o'
            || word[word.len() - 1] == b'u')
    {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ];
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        let output = vec![2, 3, 0];
        let result = Solution::vowel_strings(words, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec!["a".to_string(), "e".to_string(), "i".to_string()];
        let queries = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
        let output = vec![3, 2, 1];
        let result = Solution::vowel_strings(words, queries);
        assert_eq!(result, output);
    }
}
