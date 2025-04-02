// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        for i in 0..nums.len() {
            let a = nums[i] as i64;
            for j in i + 1..nums.len() {
                let b = nums[j] as i64;
                for k in j + 1..nums.len() {
                    let cur_result = (a - b) * nums[k] as i64;
                    result = result.max(cur_result.max(0));
                }
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
        let nums = vec![12, 6, 1, 2, 7];
        let output = 77;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 10, 3, 4, 19];
        let output = 133;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3];
        let output = 0;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }
}
