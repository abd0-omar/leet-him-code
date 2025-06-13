// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        // minimize maximize on maximum minimum -> binary search on answer
        // straight ahead
        let n = nums.len();
        nums.sort_unstable();
        let mut l = 0;
        let mut r = nums[n - 1] - nums[0];
        let mut ans = -1;
        // [1, 1, 2, 3, 7, 10]
        // because we need to minimize, we need to go to the left
        while l <= r {
            let mid = l + (r - l) / 2;
            // if possible, take ans and go left
            dbg!(&mid, l, r, ans);
            if possible(&nums, mid, p) {
                r = mid - 1;
                ans = mid;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

fn possible(nums: &[i32], mid: i32, p: i32) -> bool {
    let mut count = 0;
    let mut idx = 0;
    while idx < nums.len() - 1 {
        if nums[idx + 1] - nums[idx] <= mid {
            idx += 1;
            count += 1;
        }
        idx += 1;
    }
    count >= p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let p = 2;
        let output = 1;
        let result = Solution::minimize_max(nums, p);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 2, 1, 2];
        let p = 1;
        let output = 0;
        let result = Solution::minimize_max(nums, p);
        assert_eq!(result, output);
    }
}
