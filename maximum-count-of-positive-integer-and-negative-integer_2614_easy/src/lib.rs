// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        // find last negative, or first 0
        // find first positive, or last 0

        // let n = nums.len();
        // let (mut l, mut r) = (0, n - 1);
        // let mut negative_count = None;
        // while l <= r {
        //     let mid = l + (r - l) / 2;
        //     if nums[mid] >= 0 {
        //         if let (_, true) = mid.overflowing_sub(1) {
        //             break;
        //         }
        //         r = mid - 1;
        //     } else {
        //         negative_count = Some(mid as i32);
        //         l = mid + 1;
        //     }
        // }
        // let (mut l, mut r) = (0, n - 1);
        // let mut positive_count = None;
        // while l <= r {
        //     let mid = l + (r - l) / 2;
        //     if nums[mid] > 0 {
        //         positive_count = Some(mid as i32);
        //         if let (_, true) = mid.overflowing_sub(1) {
        //             break;
        //         }
        //         r = mid - 1;
        //     } else {
        //         l = mid + 1;
        //     }
        // }
        // // dbg!(&negative_count);
        // // dbg!(&positive_count);
        // positive_count
        //     .map(|pos| n as i32 - pos)
        //     .unwrap_or(0)
        //     .max(negative_count.map(|neg| neg + 1).unwrap_or(0))
        // works but rust has this built in

        // partition point is always used with "|&x| x <" or "|&x| x<="
        let negative_count = nums.partition_point(|&x| x < 0);
        let positive_count = nums.len() - nums.partition_point(|&x| x < 1);
        negative_count.max(positive_count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![-2, -1, -1, 1, 2, 3];
        let output = 3;
        let result = Solution::maximum_count(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![-3, -2, -1, 0, 0, 1, 2];
        let output = 3;
        let result = Solution::maximum_count(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![5, 20, 66, 1314];
        let output = 4;
        let result = Solution::maximum_count(nums);
        assert_eq!(result, output);
    }
}
