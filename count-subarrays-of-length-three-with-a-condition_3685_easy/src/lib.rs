// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        // fixed sliding window
        let n = nums.len();
        // a queue for حنيكة
        let mut queue = std::collections::VecDeque::with_capacity(3);
        // I solved it with a queue so `l` is redundant
        // let mut l = 0;
        let mut result = 0;
        for r in 0..n {
            if r >= 3 {
                queue.pop_front();
            }
            queue.push_back(nums[r]);
            if queue.len() == 3 {
                if queue[2] + queue[0] == queue[1] / 2
                    && (queue[1] as f32 / 2.0).floor() == queue[1] as f32 / 2.0
                {
                    result += 1;
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
        let nums = vec![1, 2, 1, 4, 1];
        let output = 1;
        let result = Solution::count_subarrays(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1];
        let output = 0;
        let result = Solution::count_subarrays(nums);
        assert_eq!(result, output);
    }
}
