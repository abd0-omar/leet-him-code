// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        // same as maximum subarray but with an added max negative sum
        let mut running_max_sum_pos = i32::MIN;
        let mut result_max_sum_pos = i32::MIN;
        for &num in nums.iter() {
            // two choices
            // continue with num + max_sum_pos
            // or
            // start from num

            running_max_sum_pos = match num.checked_add(running_max_sum_pos) {
                Some(sum) => sum.max(num),
                None => num,
            };
            result_max_sum_pos = result_max_sum_pos.max(running_max_sum_pos);
            // dbg!(result_max_sum_pos);
        }
        let mut running_min_sum_neg = i32::MAX;
        let mut result_min_sum_neg = i32::MAX;
        for num in nums.into_iter() {
            // two choices
            // continue with num + max_sum_neg
            // or
            // start from num

            running_min_sum_neg = match num.checked_add(running_min_sum_neg) {
                Some(sum) => sum.min(num),
                None => num,
            };
            result_min_sum_neg = result_min_sum_neg.min(running_min_sum_neg);
            // dbg!(result_min_sum_neg);
        }
        result_max_sum_pos.max(result_min_sum_neg.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, -3, 2, 3, -4];
        let output = 5;
        let result = Solution::max_absolute_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, -5, 1, -4, 3, -2];
        let output = 8;
        let result = Solution::max_absolute_sum(nums);
        assert_eq!(result, output);
    }
}
