// https://leetcode.com/problems/zero-array-transformation-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        // prefix sum on range, difference array
        let n = nums.len();
        let mut diff_array = vec![0; n + 1];
        for query in queries {
            // r inclusive
            let (l, r) = (query[0] as usize, query[1] as usize);
            diff_array[l] -= 1;
            diff_array[r + 1] += 1;
        }
        // prefix sum on diff array
        // ignore first element in the prefix_sum
        let mut prefix_sum = vec![0; n + 1];
        for i in 1..n + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + diff_array[i - 1];
        }
        // dbg!(&diff_array);
        // dbg!(&prefix_sum);
        for i in 0..n {
            nums[i] += prefix_sum[i + 1];
        }
        nums.iter().all(|&x| x < 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 0, 1];
        let queries = vec![vec![0, 2]];
        let output = true;
        let result = Solution::is_zero_array(nums, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3], vec![0, 2]];
        let output = false;
        let result = Solution::is_zero_array(nums, queries);
        assert_eq!(result, output);
    }
}
