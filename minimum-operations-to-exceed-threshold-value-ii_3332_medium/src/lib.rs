// https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        let mut heap = std::collections::BinaryHeap::from_iter(
            nums.into_iter().map(|num| Reverse(num as u64)),
        );
        let mut count = 0;
        while let Some(Reverse(pop1)) = heap.pop() {
            if pop1 >= k as u64 {
                break;
            }
            // let pop2 = match heap.pop() {
            //     Some(Reverse(pop2)) => pop2,
            //     None => break,
            // };
            let pop2 = heap.pop().unwrap().0;
            // dbg!(pop1);
            // dbg!(pop2);
            heap.push(Reverse(pop1.min(pop2) * 2 + pop1.max(pop2)));
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 11, 10, 1, 3];
        let k = 10;
        let output = 2;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 2, 4, 9];
        let k = 20;
        let output = 4;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, output);
    }
}
