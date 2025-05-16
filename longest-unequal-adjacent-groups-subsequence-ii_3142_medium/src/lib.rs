// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut memo = vec![None; n];
        let mut best = Vec::new();
        for i in 0..n {
            let seq = lis(&words, &groups, i, &mut memo);
            if seq.len() > best.len() || (seq.len() == best.len() && seq < best) {
                best = seq;
            }
        }
        best
    }
}

fn lis(
    words: &[String],
    groups: &[i32],
    i: usize,
    memo: &mut Vec<Option<Vec<String>>>,
) -> Vec<String> {
    if let Some(res) = &memo[i] {
        return res.clone();
    }
    let mut best = vec![words[i].clone()];
    for j in 0..i {
        if groups[j] != groups[i]
            && is_hamming_distance_one(words[i].as_bytes(), words[j].as_bytes())
        {
            let mut seq = lis(words, groups, j, memo);
            seq.push(words[i].clone());
            if seq.len() > best.len() || (seq.len() == best.len() && seq < best) {
                best = seq;
            }
        }
    }
    memo[i] = Some(best.clone());
    best
}

fn is_hamming_distance_one(str1: &[u8], str2: &[u8]) -> bool {
    if str1.len() != str2.len() {
        return false;
    }
    let mut diff_count = 0;
    for i in 0..str1.len() {
        if str1[i] != str2[i] {
            diff_count += 1;
        }
        if diff_count > 1 {
            return false;
        }
    }
    diff_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec!["bab".to_string(), "dab".to_string(), "cab".to_string()];
        let groups = vec![1, 2, 2];
        let output = vec!["bab".to_string(), "cab".to_string()];
        let result = Solution::get_words_in_longest_subsequence(words, groups);
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
        let groups = vec![1, 2, 3, 4];
        let output = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let result = Solution::get_words_in_longest_subsequence(words, groups);
        assert_eq!(result, output);
    }
}
