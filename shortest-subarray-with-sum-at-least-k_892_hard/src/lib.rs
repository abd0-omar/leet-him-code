// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        // sliding window won't work cuz of the negative numbers
        // O(n^2) solution is trivial but won't pass
        let mut result = i32::MAX;
        let mut cur_sum = 0i64;
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        // (prefix_sum, idx)
        let mut min_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        for r in 0..nums.len() {
            cur_sum += nums[r] as i64;

            if cur_sum >= k as i64 {
                result = result.min(r as i32 + 1);
            }

            while !min_heap.is_empty() && cur_sum - min_heap.peek().unwrap().0 .0 as i64 >= k as i64
            {
                let Reverse((_prefix, end_idx)) = min_heap.pop().unwrap();
                result = result.min((r - end_idx) as i32);
            }
            min_heap.push(Reverse((cur_sum as i64, r)));
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1];
        let k = 1;
        let output = 1;
        let result = Solution::shortest_subarray(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2];
        let k = 4;
        let output = -1;
        let result = Solution::shortest_subarray(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![2, -1, 2];
        let k = 3;
        let output = 3;
        let result = Solution::shortest_subarray(nums, k);
        assert_eq!(result, output);
    }
}
