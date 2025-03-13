// https://leetcode.com/problems/zero-array-transformation-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // prefix range difference array pattern, it's easy to identify when you
        // have range(l, r) and a val to increment or decrement the targeted
        // array

        // but this problem is binary search on answer (k) and diff prefix array
        if !diff_array_prefix_sum_for_range_queries(&nums, &queries, queries.len()) {
            // dbg!("a7a?!");
            return -1;
        }

        let (mut l, mut r) = (0, queries.len());
        let mut ans = -1;

        // we want first T
        while l <= r {
            let mid = l + (r - l) / 2;
            if diff_array_prefix_sum_for_range_queries(&nums, &queries, mid) {
                ans = mid as i32;
                if let (_, true) = mid.overflowing_sub(1) {
                    break;
                }
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

fn diff_array_prefix_sum_for_range_queries(
    nums: &Vec<i32>,
    queries: &Vec<Vec<i32>>,
    k: usize,
) -> bool {
    let n = nums.len();
    // the extra (+ 1) is not needed, you can stop at n if you want and
    // that's what I will do here
    let mut diff_array = vec![0; n];

    for q in queries.iter().take(k) {
        let (l, r, val) = (q[0] as usize, q[1] as usize, q[2]);

        diff_array[l] -= val;
        if r + 1 == n {
            continue;
        }
        diff_array[r + 1] += val;
        // dbg!(&diff_array);
    }

    // after computing diff array, do prefix sum on it (the diff array)
    let mut prefix_sum = vec![0; n];
    prefix_sum[0] = diff_array[0];
    for i in 1..n {
        prefix_sum[i] = prefix_sum[i - 1] + diff_array[i];
    }
    // dbg!(&diff_array);
    // dbg!(&prefix_sum);

    // check if the final array will be all zeros
    for i in 0..n {
        let new_val = nums[i] + prefix_sum[i];
        // dbg!(&prefix_sum[i]);
        // dbg!(&nums[i]);
        // dbg!(&new_val);
        if new_val > 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]];
        let output = 2;
        let result = Solution::min_zero_array(nums, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3, 2], vec![0, 2, 1]];
        let output = -1;
        let result = Solution::min_zero_array(nums, queries);
        assert_eq!(result, output);
    }
}
