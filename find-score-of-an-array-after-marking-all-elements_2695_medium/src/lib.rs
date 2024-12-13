// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut mark = vec![false; n];
        use std::cmp::Reverse;
        // storing prev and next is unnecessary and could use only cur_idx to
        // determine both prev and next
        let mut min_heap = std::collections::BinaryHeap::new();
        // Fence Post Problem..I think
        if n == 1 {
            min_heap.push((Reverse(nums[0]), Reverse(0), (0, 0, 0)));
        } else {
            for i in 0..n {
                if i == 0 {
                    // first idx marking edge case
                    // mark it again would do nothing wrong

                    // `Reverse(i)` to make the sorting algorithm stable
                    min_heap.push((Reverse(nums[i]), Reverse(i), (i, i, i + 1)));
                } else if i == n - 1 {
                    min_heap.push((Reverse(nums[i]), Reverse(i), (i - 1, i, i)));
                } else {
                    min_heap.push((Reverse(nums[i]), Reverse(i), (i - 1, i, i + 1)));
                }
            }
        }
        let mut result_sum = 0;
        while let Some((Reverse(top), _idx, (prev, curr, next))) = min_heap.pop() {
            if mark[curr] {
                continue;
            }
            (mark[prev], mark[curr], mark[next]) = (true, true, true);
            result_sum += top as i64;
        }
        result_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 1, 3, 4, 5, 2];
        let output = 7;
        let result = Solution::find_score(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 3, 5, 1, 3, 2];
        let output = 5;
        let result = Solution::find_score(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3];
        let output = 3;
        let result = Solution::find_score(nums);
        assert_eq!(result, output);
    }
}
