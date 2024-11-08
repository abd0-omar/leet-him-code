// https://leetcode.com/problems/maximum-xor-for-each-query/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        // TLE
        // let k_max_bound = 2i32.pow(maximum_bit as u32);
        //
        // let mut xor_result = 0;
        // for i in 0..nums.len() {
        //     xor_result ^= nums[i];
        // }
        // // dbg!(xor_result);
        //
        // let mut result = Vec::new();
        // let mut n = nums.len();
        // for _ in 0..n {
        //     let mut max_xor = 0;
        //     let mut k_max = 0;
        //     for k in 0..k_max_bound {
        //         let f = k ^ xor_result;
        //         // dbg!(f);
        //         // dbg!(k);
        //         if f > max_xor {
        //             k_max = k;
        //             max_xor = f;
        //         }
        //     }
        //     xor_result ^= nums[n - 1];
        //     n -= 1;
        //     result.push(k_max);
        // }
        // result
        let k_max_bound = 2i32.pow(maximum_bit as u32) - 1;
        let n = nums.len();
        let mut xor_result = 0;

        for &num in &nums {
            xor_result ^= num;
        }

        let mut result = Vec::with_capacity(n);

        for i in (0..n).rev() {
            result.push(xor_result ^ k_max_bound);

            // remove last num by updating xor_result
            xor_result ^= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![0, 1, 1, 3];
        let maximum_bit = 2;
        let output = vec![0, 3, 2, 3];
        let result = Solution::get_maximum_xor(nums, maximum_bit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 3, 4, 7];
        let maximum_bit = 3;
        let output = vec![5, 2, 6, 5];
        let result = Solution::get_maximum_xor(nums, maximum_bit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![0, 1, 2, 2, 5, 7];
        let maximum_bit = 3;
        let output = vec![4, 3, 6, 4, 6, 7];
        let result = Solution::get_maximum_xor(nums, maximum_bit);
        assert_eq!(result, output);
    }
}
