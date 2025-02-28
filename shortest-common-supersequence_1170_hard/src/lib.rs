// https://leetcode.com/problems/shortest-common-supersequence/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let m = str1.len();
        let n = str2.len();
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Compute LCS lengths
        for i in 0..m {
            for j in 0..n {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        // Reconstruct the SCS from LCS
        let mut result = Vec::with_capacity(m + n);
        let (mut i, mut j) = (m, n);
        while i > 0 || j > 0 {
            if i > 0 && j > 0 && s1[i - 1] == s2[j - 1] {
                result.push(s1[i - 1]);
                i -= 1;
                j -= 1;
            } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
                result.push(s2[j - 1]);
                j -= 1;
            } else if i > 0 {
                result.push(s1[i - 1]);
                i -= 1;
            }
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

// impl Solution {
//     pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
//         // like edit distance problem, but it gets memory limit exceeded
//         let str1 = str1.chars().collect::<Vec<char>>();
//         let str2 = str2.chars().collect::<Vec<char>>();
//         let mut memory = vec![vec![None; str2.len() + 1]; str1.len() + 1];
//         let result = backtrack(0, 0, &str1, &str2, &mut memory);
//         result.into_iter().collect()
//     }
// }
//
// fn backtrack(
//     i: usize,
//     j: usize,
//     str1: &Vec<char>,
//     str2: &Vec<char>,
//     memory: &mut Vec<Vec<Option<Vec<char>>>>,
// ) -> Vec<char> {
//     if i == str1.len() {
//         return str2[j..].to_vec();
//     }
//     if j == str2.len() {
//         return str1[i..].to_vec();
//     }

//     if let Some(result) = &memory[i][j] {
//         return result.clone();
//     }

//     let result = if str1[i] == str2[j] {
//         let mut res = vec![str1[i]];
//         res.extend_from_slice(&backtrack(i + 1, j + 1, str1, str2, memory));
//         res
//     } else {
//         let mut choice1 = vec![str1[i]];
//         choice1.extend_from_slice(&backtrack(i + 1, j, str1, str2, memory));

//         let mut choice2 = vec![str2[j]];
//         choice2.extend_from_slice(&backtrack(i, j + 1, str1, str2, memory));

//         if choice1.len() < choice2.len() {
//             choice1
//         } else {
//             choice2
//         }
//     };

//     memory[i][j] = Some(result.clone());
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let str1 = "abac".to_string();
        let str2 = "cab".to_string();
        let output = "cabac".to_string();
        let result = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let str1 = "aaaaaaaa".to_string();
        let str2 = "aaaaaaaa".to_string();
        let output = "aaaaaaaa".to_string();
        let result = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(result, output);
    }
}
