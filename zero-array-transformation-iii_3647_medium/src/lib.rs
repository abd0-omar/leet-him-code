// https://leetcode.com/problems/zero-array-transformation-iii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        // the legend's solution
        // https://leetcode.com/problems/zero-array-transformation-iii/solutions/6768535/priority-queues-in-depth-with-images-idea-behind-solution-c-python-java
        queries.sort_unstable();
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut available = BinaryHeap::new();
        let mut assigned = BinaryHeap::new();
        let mut count = 0;
        let mut k = 0;
        let n = nums.len();
        for time in 0..n {
            while let Some(&Reverse(top)) = assigned.peek() {
                if top < time {
                    assigned.pop();
                } else {
                    break;
                }
            }
            while k < queries.len() && queries[k][0] <= time as i32 {
                available.push(queries[k][1]);
                k += 1;
            }
            while assigned.len() < nums[time] as usize
                && !available.is_empty()
                && *available.peek().unwrap() >= time as i32
            {
                assigned.push(Reverse(available.pop().unwrap() as usize));
                count += 1;
            }
            if assigned.len() < nums[time] as usize {
                return -1;
            }
        }
        (queries.len() - count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2], vec![0, 2], vec![1, 1]];
        let output = 1;
        let result = Solution::max_removal(nums, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1, 1];
        let queries = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]];
        let output = 2;
        let result = Solution::max_removal(nums, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![0, 3]];
        let output = -1;
        let result = Solution::max_removal(nums, queries);
        assert_eq!(result, output);
    }
}
