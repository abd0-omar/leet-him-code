use std::collections::HashSet;

// https://leetcode.com/problems/find-unique-binary-string/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        // will generate all binary strings and look it up in nums
        let n = nums[0].len();
        let nums: HashSet<String> = HashSet::from_iter(nums.into_iter());
        let binary_nums = ['0', '1'];
        let mut answer = String::new();
        backtrack(
            n,
            String::with_capacity(n),
            &binary_nums,
            &nums,
            &mut answer,
        );
        answer
    }
}

fn backtrack(
    n: usize,
    mut cur: String,
    binary_nums: &[char],
    nums: &HashSet<String>,
    answer: &mut String,
) {
    if !answer.is_empty() {
        return;
    }

    if cur.len() == n {
        if !nums.contains(&cur) {
            *answer = cur;
        }
        return;
    }

    for &binary in binary_nums {
        cur.push(binary);
        backtrack(n, cur.clone(), binary_nums, nums, answer);
        cur.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec!["01".to_string(), "10".to_string()];
        let output = "11".to_string();
        let result = Solution::find_different_binary_string(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec!["00".to_string(), "01".to_string()];
        let output = "11".to_string();
        let result = Solution::find_different_binary_string(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        let output = "101".to_string();
        let result = Solution::find_different_binary_string(nums);
        assert_eq!(result, output);
    }
}
