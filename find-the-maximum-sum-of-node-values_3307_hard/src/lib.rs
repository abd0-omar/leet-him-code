// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let n = nums.len();
        let mut memo = vec![vec![-1i64; 2]; n];
        max_sum_of_nodes(0, 1, &nums, k, &mut memo)
    }
}

fn max_sum_of_nodes(
    index: usize,
    is_even: usize,
    nums: &Vec<i32>,
    k: i32,
    memo: &mut Vec<Vec<i64>>,
) -> i64 {
    if index == nums.len() {
        return if is_even == 1 { 0 } else { i64::MIN };
    }
    if memo[index][is_even] != -1 {
        return memo[index][is_even];
    }
    // leave (no xor)
    let no_xor_done = nums[index] as i64 + max_sum_of_nodes(index + 1, is_even, nums, k, memo);
    // take (do xor)
    let xor_done =
        (nums[index] ^ k) as i64 + max_sum_of_nodes(index + 1, (is_even + 1) % 2, nums, k, memo);

    let res = no_xor_done.max(xor_done);
    memo[index][is_even] = res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 1];
        let k = 3;
        let edges = vec![vec![0, 1], vec![0, 2]];
        let output = 6;
        let result = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 3];
        let k = 7;
        let edges = vec![vec![0, 1]];
        let output = 9;
        let result = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![7, 7, 7, 7, 7, 7];
        let k = 3;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
        let output = 42;
        let result = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, output);
    }
}
