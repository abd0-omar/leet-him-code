// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        // greedy
        // always pick the first word
        let mut result = Vec::new();
        let n = words.len();
        result.push(words[0].clone());
        let mut cur_group = groups[0];
        for i in 1..n {
            if cur_group != groups[i] {
                result.push(words[i].clone());
                cur_group = groups[i];
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
        let words = vec!["e".to_string(), "a".to_string(), "b".to_string()];
        let groups = vec![0, 0, 1];
        let output = vec!["e".to_string(), "b".to_string()];
        let result = Solution::get_longest_subsequence(words, groups);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let groups = vec![1, 0, 1, 1];
        let output = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = Solution::get_longest_subsequence(words, groups);
        assert_eq!(result, output);
    }
}
