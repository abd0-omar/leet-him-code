// https://leetcode.com/problems/bitwise-xor-of-all-pairings/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // [2, 1, 3]
        // [10, 2, 5, 0]
        /*
        2 ^ 10, 2 ^ 2, 2 ^ 5, 2 ^ 0
        2 ^ 10 ^ 2 ^ 2 ^ 2 ^ 5 ^ 2 ^ 0
        x ^ 10 ^ x ^ 2 ^ x ^ 5 ^ x ^ 0
        all 2s from nums1 will be removed because they occur at even number
        if nums1.len is odd and nums2.len is even

        // even - odd
        // [2, 1, 3, 4]
        // [10, 2, 5]
        2 ^ 10, 2 ^ 2, 2 ^ 5
        2 ^ 10 ^ 2 ^ 2 ^ 2 ^ 5
        2 ^ 10 ^ x ^ 2 ^ x ^ 5
        only one 2 remains

        // even - even (like test case no.2) would be zero

        // odd - odd
        // [2, 1, 3]
        // [10, 2, 5]
        2 ^ 10, 2 ^ 2, 2 ^ 5
        2 ^ 10 ^ 2 ^ 2 ^ 2 ^ 5
        2 ^ 10 ^ x ^ 2 ^ x ^ 5
        only one 2 remains
        */
        let mut xor = 0;
        let n1 = nums1.len();
        let n2 = nums2.len();

        if n2 % 2 == 1 {
            for num in nums1 {
                xor ^= num
            }
        }

        if n1 % 2 == 1 {
            for num in nums2 {
                xor ^= num
            }
        }

        xor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums1 = vec![2, 1, 3];
        let nums2 = vec![10, 2, 5, 0];
        let output = 13;
        let result = Solution::xor_all_nums(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let output = 0;
        let result = Solution::xor_all_nums(nums1, nums2);
        assert_eq!(result, output);
    }
}
