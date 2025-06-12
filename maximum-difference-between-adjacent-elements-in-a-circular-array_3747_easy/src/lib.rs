// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut result: Option<i32> = None;
        for window in nums.windows(2) {
            let diff = (window[1] - window[0]).abs();
            if let Some(res) = result {
                result = Some(res.max(diff));
            } else {
                result = Some(diff);
            }
            dbg!(&diff);
            dbg!(&window);
        }
        dbg!(nums[nums.len() - 1] - nums[0]);
        result = Some(result.unwrap().max((nums[nums.len() - 1] - nums[0]).abs()));
        result.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 4];
        let output = 3;
        let result = Solution::max_adjacent_distance(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![-5, -10, -5];
        let output = 5;
        let result = Solution::max_adjacent_distance(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![-2, -5];
        let output = 3;
        let result = Solution::max_adjacent_distance(nums);
        assert_eq!(result, output);
    }
}
