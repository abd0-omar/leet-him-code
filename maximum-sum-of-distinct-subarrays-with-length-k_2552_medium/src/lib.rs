// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut count = 0;
        use std::collections::HashSet;
        let mut hs = HashSet::new();
        let mut st = 0;
        let mut end = 0;
        let mut max_sum = 0i64;
        let mut cur_sum = 0i64;

        while end < n {
            count += 1;
            if count < k {
                if !hs.insert(nums[end]) {
                    count = 0;
                    hs.clear();
                    st += 1;
                    end = st;
                    cur_sum = 0;
                    continue;
                }
                cur_sum += nums[end] as i64;
            } else if count == k {
                if !hs.insert(nums[end]) {
                    count = 0;
                    hs.clear();
                    st += 1;
                    end = st;
                    cur_sum = 0;
                    continue;
                }
                cur_sum += nums[end] as i64;
                max_sum = max_sum.max(cur_sum);
            } else {
                hs.remove(&nums[st]);
                if !hs.insert(nums[end]) {
                    count = 0;
                    hs.clear();
                    st += 1;
                    end = st;
                    continue;
                }
                cur_sum -= nums[st] as i64;
                cur_sum += nums[end] as i64;
                max_sum = max_sum.max(cur_sum);
                st += 1;
            }
            end += 1;
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;
        let output = 15;
        let result = Solution::maximum_subarray_sum(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 4, 4];
        let k = 3;
        let output = 0;
        let result = Solution::maximum_subarray_sum(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 1, 1, 7, 8, 9];
        let k = 3;
        let output = 24;
        let result = Solution::maximum_subarray_sum(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![1, 1, 2];
        let k = 2;
        let output = 3;
        let result = Solution::maximum_subarray_sum(nums, k);
        assert_eq!(result, output);
    }
}
