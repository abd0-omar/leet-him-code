// https://leetcode.com/problems/number-of-ways-to-split-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        // prefix_sum from end (reversed)
        let n = nums.len();
        let mut prefix_sum_rev = vec![0i64; n + 1];
        // self-note
        // delete line in helix is fater than nvim, xd vs Vd or even dd

        // we dont need the first prefix_sum because the first split will always have the first num
        for i in (1..n).rev() {
            prefix_sum_rev[i] = prefix_sum_rev[i + 1] + nums[i] as i64;
        }
        // dbg!(&prefix_sum_rev);
        // 13, 3, -1, 7
        let mut cur_sum = 0i64;
        let mut result_count = 0;
        for i in 0..n - 1 {
            cur_sum += nums[i] as i64;
            // let other_sum = prefix_sum_rev[i + 1];
            // dbg!(&cur_sum);
            // dbg!(&other_sum);
            if cur_sum >= prefix_sum_rev[i + 1] as i64 {
                // dbg!("wow");
                result_count += 1;
            }
        }
        result_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![10, 4, -8, 7];
        let output = 2;
        let result = Solution::ways_to_split_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 3, 1, 0];
        let output = 2;
        let result = Solution::ways_to_split_array(nums);
        assert_eq!(result, output);
    }
}
