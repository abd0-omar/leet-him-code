// https://leetcode.com/problems/trapping-rain-water-ii/
#[allow(dead_code)]
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();

        let mut min_heap = BinaryHeap::new();
        let mut visited = vec![vec![false; m]; n];

        for j in 0..m {
            min_heap.push((Reverse(height_map[0][j]), 0, j));
            min_heap.push((Reverse(height_map[n - 1][j]), n - 1, j));
            visited[0][j] = true;
            visited[n - 1][j] = true;
        }
        for i in 0..n {
            min_heap.push((Reverse(height_map[i][0]), i, 0));
            min_heap.push((Reverse(height_map[i][m - 1]), i, m - 1));
            visited[i][0] = true;
            visited[i][m - 1] = true;
        }

        let mut cur_max_height = 0;
        let mut result = 0;

        while let Some((Reverse(height), i, j)) = min_heap.pop() {
            cur_max_height = cur_max_height.max(height);

            result += cur_max_height - height;

            for (di, dj) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let ni = i.wrapping_add(di as usize);
                let nj = j.wrapping_add(dj as usize);

                if (0..n).contains(&ni) && (0..m).contains(&nj) && !visited[ni][nj] {
                    min_heap.push((Reverse(height_map[ni][nj]), ni, nj));
                    visited[ni][nj] = true;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let output = 4;
        let result = Solution::trap_rain_water(height_map);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let output = 10;
        let result = Solution::trap_rain_water(height_map);
        assert_eq!(result, output);
    }
}
