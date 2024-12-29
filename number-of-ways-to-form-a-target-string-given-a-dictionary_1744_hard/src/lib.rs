// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
use std::collections::HashMap;
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let mut freq = HashMap::new();
        let words: Vec<Vec<u8>> = words.into_iter().map(|word| word.into_bytes()).collect();
        for word in words.iter() {
            for (idx, &letter) in word.into_iter().enumerate() {
                *freq.entry((idx, letter)).or_insert(0) += 1;
            }
        }
        let mut memory = HashMap::new();
        dfs(0, 0, target.as_bytes(), &words, &freq, &mut memory)
    }
}

fn dfs(
    i: usize,
    k: usize,
    target: &[u8],
    words: &Vec<Vec<u8>>,
    freq: &HashMap<(usize, u8), i32>,
    memory: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if i == target.len() {
        return 1;
    }
    if k == words[0].len() {
        return 0;
    }
    if let Some(ret) = memory.get(&(i, k)) {
        return *ret;
    }
    // try at every word that has target[i] at the current k position
    let take = if let Some(f) = freq.get(&(k, target[i])) {
        let prod = (*f as i64) * dfs(i + 1, k + 1, target, words, &freq, memory) as i64;
        (prod % MOD as i64) as i32
    } else {
        0
    };
    let leave = dfs(i, k + 1, target, words, &freq, memory);
    let result = (take + leave) % MOD;
    memory.insert((i, k), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
        let target = "aba".to_string();
        let output = 6;
        let result = Solution::num_ways(words, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec!["abba".to_string(), "baab".to_string()];
        let target = "bab".to_string();
        let output = 4;
        let result = Solution::num_ways(words, target);
        assert_eq!(result, output);
    }
}
