// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::<(Reverse<i32>, usize, usize)>::from([(Reverse(0), 0, 0)]);

        // visited
        let n = move_time.len();
        let m = move_time[0].len();
        let mut visited = vec![vec![false; m]; n];

        while let Some((Reverse(t), i, j)) = pq.pop() {
            for (ni, nj) in [
                (i + 1, j),
                (i, j + 1),
                (i.wrapping_sub(1), j),
                (i, j.wrapping_sub(1)),
            ] {
                if ni < n && nj < m && !visited[ni][nj] {
                    let nt = t.max(move_time[ni][nj]) + (ni + nj) as i32 % 2 + 1;
                    if ni == n - 1 && nj == m - 1 {
                        return nt;
                    }
                    pq.push((Reverse(nt), ni, nj));
                    visited[ni][nj] = true;
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
        let output = 7;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let move_time = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        let output = 6;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let move_time = vec![vec![0, 1], vec![1, 2]];
        let output = 4;
        let result = Solution::min_time_to_reach(move_time);
        assert_eq!(result, output);
    }
}
