// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        // overlapping intervals
        nums.sort_unstable();
        // binary search on the result
        // T T T F F F
        // we want the last true
        let mut l = 1;
        let mut r = nums.len();
        let mut ans = -1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(&nums, mid, k) {
                l = mid + 1;
                ans = mid as i32;
            } else {
                if let (_, true) = mid.overflowing_sub(1) {
                    break;
                }
                r = mid - 1;
            }
        }

        ans
    }
}

fn possible(nums: &[i32], candidate_len: usize, k: i32) -> bool {
    // window
    let mut st = 0;
    let mut end = 0;
    while end < nums.len() {
        if end + 1 > candidate_len {
            st += 1;
        }
        if nums[st] + k >= nums[end] - k && end - st + 1 == candidate_len {
            return true;
        }
        end += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![4, 6, 1, 2];
        let k = 2;
        let output = 3;
        let result = Solution::maximum_beauty(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1, 1];
        let k = 10;
        let output = 4;
        let result = Solution::maximum_beauty(nums, k);
        assert_eq!(result, output);
    }
}
