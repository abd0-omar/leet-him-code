use std::collections::HashSet;

// https://leetcode.com/problems/count-complete-subarrays-in-an-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        // at least k pattern

        let k = count_distinct(&nums);

        let mut l = 0;
        let n = nums.len();
        let mut result = 0;
        for r in 0..n {
            // count_distinct
            let mut distinct_count = count_distinct(&nums[l..=r]);
            // while valid
            while distinct_count == k {
                // update result
                result += n - r;
                // move l
                // distinct_elements.remove(&nums[l]);
                l += 1;
                distinct_count = count_distinct(&nums[l..=r]);
                // count distinct
            }
            // dbg!(&l);
            // dbg!(&result);
            // dbg!(&distinct_elements);
        }
        result as i32
    }
}

fn count_distinct(nums: &[i32]) -> usize {
    nums.iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3, 1, 2, 2];
        let output = 4;
        let result = Solution::count_complete_subarrays(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![5, 5, 5, 5];
        let output = 10;
        let result = Solution::count_complete_subarrays(nums);
        assert_eq!(result, output);
    }
}
