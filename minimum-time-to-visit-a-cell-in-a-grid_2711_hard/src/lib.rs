// https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        if grid[0][1].min(grid[1][0]) > 1 {
            return -1;
        }
        use std::cmp::Reverse;
        let mut pq = std::collections::BinaryHeap::from([(Reverse(0), 0, 0)]);
        let mut visited = vec![vec![false; m]; n];

        while !pq.is_empty() {
            let size = pq.len();
            for _ in 0..size {
                let cur_cell = pq.pop().unwrap();
                dbg!(&cur_cell);
                visited[cur_cell.1][cur_cell.2] = true;
                if cur_cell.1 == n - 1 && cur_cell.2 == m - 1 {
                    return cur_cell.0 .0;
                }

                for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let ni = cur_cell.1.wrapping_add(di as usize);
                    let nj = cur_cell.2.wrapping_add(dj as usize);

                    if (0..n).contains(&ni) && (0..m).contains(&nj) && !visited[ni][nj] {
                        visited[ni][nj] = true;
                        let wait = {
                            if (grid[ni][nj] - cur_cell.0 .0).abs() % 2 == 0 {
                                1
                            } else {
                                0
                            }
                        };
                        let new_time = (grid[ni][nj] + wait).max(cur_cell.0 .0 + 1);
                        pq.push((Reverse(new_time), ni, nj));
                    }
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
        let grid = vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]];
        let output = 7;
        let result = Solution::minimum_time(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]];
        let output = -1;
        let result = Solution::minimum_time(grid);
        assert_eq!(result, output);
    }
}
