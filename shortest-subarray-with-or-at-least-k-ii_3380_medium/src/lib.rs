// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        // prefix_or with binary search
        let n = nums.len();
        let mut prefix_or = vec![0; n + 1];

        for i in 1..n + 1 {
            prefix_or[i] = prefix_or[i - 1] | nums[i - 1];
        }

        let mut l = 1;
        let mut r = n;
        let mut ans = -1;

        while l <= r {
            // potentail answer
            let mid = l + (r - l) / 2;

            if Self::check_subarray(mid, &nums, k) {
                ans = mid as i32;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        ans
    }

    fn check_subarray(potential_len: usize, nums: &[i32], k: i32) -> bool {
        let mut freq = [0; 32];

        let n = nums.len();
        for end in 0..n {
            // add 1 bits to the freq
            let binary_end = format!("{:b}", nums[end]);
            for (idx, bit) in binary_end.chars().rev().enumerate() {
                if bit == '1' {
                    freq[32 - 1 - idx] += 1;
                }
            }

            if end >= potential_len {
                let binary_st = format!("{:b}", nums[end - potential_len]);
                for (idx, bit) in binary_st.chars().rev().enumerate() {
                    if bit == '1' {
                        freq[32 - 1 - idx] -= 1;
                    }
                }
            }

            if end >= potential_len - 1 {
                let decimal = Self::convert_freq_to_decimal(&freq);
                if decimal >= k {
                    return true;
                }
            }
        }

        false
    }

    fn convert_freq_to_decimal(freq: &[i32; 32]) -> i32 {
        let mut result = 0;
        for (i, &count) in freq.iter().enumerate() {
            if count > 0 {
                // most significant bit is at index zero
                // least significant is at 32 - 1
                result += 2_i32.pow(32 - 1 - i as u32); // `2^bit_position`
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
        let nums = vec![1, 2, 3];
        let k = 2;
        let output = 1;
        let result = Solution::minimum_subarray_length(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 8];
        let k = 10;
        let output = 3;
        let result = Solution::minimum_subarray_length(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2];
        let k = 0;
        let output = 1;
        let result = Solution::minimum_subarray_length(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![1, 2, 32, 21];
        let k = 55;
        let output = 3;
        let result = Solution::minimum_subarray_length(nums, k);
        assert_eq!(result, output);
    }
}
