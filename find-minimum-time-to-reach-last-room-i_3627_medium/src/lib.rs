// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
#[allow(dead_code)]
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let mut pq = BinaryHeap::new();
        let mut visited = vec![vec![false; m]; n];
        pq.push((Reverse(0), 0, 0));
        visited[0][0] = true;
        while let Some((Reverse(time), i, j)) = pq.pop() {
            if (i, j) == (n - 1, m - 1) {
                return time;
            }
            for (ni, nj) in [
                (i + 1, j),
                (i.wrapping_sub(1), j),
                (i, j + 1),
                (i, j.wrapping_sub(1)),
            ] {
                if ni < n && nj < m && !visited[ni][nj] {
                    visited[ni][nj] = true;
                    pq.push((Reverse(1 + time.max(move_time[ni][nj])), ni, nj));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let move_time = vec![vec![0, 4], vec![4, 4]];
        let output = 6;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let move_time = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let output = 3;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let move_time = vec![vec![0, 1], vec![1, 2]];
        let output = 3;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }
}
