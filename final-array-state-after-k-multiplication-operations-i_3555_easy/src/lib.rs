// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        // a mix between these two
        // https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/
        // https://leetcode.com/problems/take-gifts-from-the-richest-pile/

        // (num, idx)
        use std::cmp::Reverse;
        let mut min_heap = std::collections::BinaryHeap::from_iter(
            nums.iter()
                .enumerate()
                .map(|(idx, &num)| Reverse((num, idx))),
        );

        for _ in 0..k {
            let Reverse((_cur_min, cur_min_idx)) = min_heap.pop().unwrap();
            nums[cur_min_idx] *= multiplier;
            min_heap.push(Reverse((nums[cur_min_idx], cur_min_idx)));
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 1, 3, 5, 6];
        let k = 5;
        let multiplier = 2;
        let output = vec![8, 4, 6, 5, 6];
        let result = Solution::get_final_state(nums, k, multiplier);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2];
        let k = 3;
        let multiplier = 4;
        let output = vec![16, 8];
        let result = Solution::get_final_state(nums, k, multiplier);
        assert_eq!(result, output);
    }
}
