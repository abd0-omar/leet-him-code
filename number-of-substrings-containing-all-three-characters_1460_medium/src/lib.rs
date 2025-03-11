// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        // it's a less intemedating version of yesterday's problem
        // at least k pattern, yesterday's with exactly k
        // it's a sliding window problem which means it can be also solved with
        // prefix sum

        // sliding-window technique
        // let mut all_three = [0; 3];
        // let s = s.as_bytes();
        // let n = s.len();
        // let mut l = 0;
        // let mut result = 0;
        // for r in 0..n {
        //     // 0 or 1 or 2
        //     let letter_idx = (s[r] - b'a') as usize;
        //     all_three[letter_idx] += 1;
        //     // dbg!(&all_three);
        //     while all_three.iter().all(|&letter_count| letter_count > 0) {
        //         let letter_idx = (s[l] - b'a') as usize;
        //         all_three[letter_idx] -= 1;
        //         l += 1;
        //         result += n - r;
        //     }
        // }
        // result as i32

        let mut prefix_sum = vec![[0; 3]; s.len() + 1];
        let s = s.as_bytes();
        let mut result = 0;
        let n = s.len();

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i].clone();
            prefix_sum[i + 1][(s[i] - b'a') as usize] += 1;
        }

        for i in 0..n {
            let mut left = i + 1;
            let mut right = n - 1;
            let mut ans = n;
            while left <= right {
                let mid = (left + right) / 2;
                if prefix_sum[mid + 1][0] - prefix_sum[i][0] > 0
                    && prefix_sum[mid + 1][1] - prefix_sum[i][1] > 0
                    && prefix_sum[mid + 1][2] - prefix_sum[i][2] > 0
                {
                    // first true
                    ans = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            // dbg!(i);
            // dbg!(ans);
            result += n - ans;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "abcabc".to_string();
        let output = 10;
        let result = Solution::number_of_substrings(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "aaacb".to_string();
        let output = 3;
        let result = Solution::number_of_substrings(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "abc".to_string();
        let output = 1;
        let result = Solution::number_of_substrings(s);
        assert_eq!(result, output);
    }
}
