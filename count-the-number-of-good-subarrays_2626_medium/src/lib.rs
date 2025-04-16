// https://leetcode.com/problems/count-the-number-of-good-subarrays/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        // at least k pattern
        // like
        // x = https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
        // there is also exact k pattern which is the one before x problem
        let mut hm = std::collections::HashMap::new();
        let n = nums.len();
        let mut l = 0;
        let mut count_pairs = 0;
        let mut result = 0;
        for r in 0..n {
            let count = hm.entry(nums[r]).or_insert(0);
            // different from x problem
            // add a pair from the right if it's count = 1, first time it's
            // count is zero so nothing happens
            count_pairs += *count;
            *count += 1;
            // while valid
            while l < r && count_pairs >= k {
                result += (n - r) as i64;
                let count = hm.get_mut(&nums[l]).unwrap();
                *count -= 1;
                // different from x problem
                // remove a pair from the left
                count_pairs -= *count;
                l += 1;
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
        let nums = vec![1, 1, 1, 1, 1];
        let k = 10;
        let output = 1;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![3, 1, 4, 3, 2, 2, 4];
        let k = 2;
        let output = 4;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, output);
    }
}
