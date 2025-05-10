// https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        // replace all zeros with 1s
        let zeros1 = nums1.iter().filter(|&x| *x == 0).count() as i64;
        let zeros2 = nums2.iter().filter(|&x| *x == 0).count() as i64;

        let sum1 = nums1.iter().map(|&x| x as i64).sum::<i64>() + zeros1;
        let sum2 = nums2.iter().map(|&x| x as i64).sum::<i64>() + zeros2;

        if (zeros1 == 0 && sum2 > sum1) || (zeros2 == 0 && sum1 > sum2) {
            return -1;
        }

        sum1.max(sum2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums1 = vec![3, 2, 0, 1, 0];
        let nums2 = vec![6, 5, 0];
        let output = 12;
        let result = Solution::min_sum(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![2, 0, 2, 0];
        let nums2 = vec![1, 4];
        let output = -1;
        let result = Solution::min_sum(nums1, nums2);
        assert_eq!(result, output);
    }
}
