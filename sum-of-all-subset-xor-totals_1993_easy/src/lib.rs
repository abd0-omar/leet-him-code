// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        // backtracking generate all subsets
        let mut all_subsets = Vec::new();
        backtrack(&nums, Vec::new(), &mut all_subsets, 0);
        // dbg!(&all_subsets);
        all_subsets
            .into_iter()
            .map(|subset| subset.into_iter().fold(0, |acc, x| acc ^ x))
            .sum()
    }
}

fn backtrack(nums: &[i32], mut cur: Vec<i32>, result: &mut Vec<Vec<i32>>, idx: usize) {
    result.push(cur.clone());
    for i in idx..nums.len() {
        cur.push(nums[i]);
        backtrack(nums, cur.clone(), result, i + 1);
        cur.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3];
        let output = 6;
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![5, 1, 6];
        let output = 28;
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 4, 5, 6, 7, 8];
        let output = 480;
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, output);
    }
}
