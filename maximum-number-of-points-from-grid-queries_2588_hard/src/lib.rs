// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        // elminate repeated work
        // I could've made a (query, idx) vec and sort it as usual but it's
        // healthy to change things a bit
        let q_len = queries.len();
        let mut queries_indecies = (0..q_len).collect::<Vec<_>>();
        queries_indecies.sort_unstable_by(|&a, &b| queries[a].cmp(&queries[b]));
        // dbg!(&queries_indecies);
        // bfs heap (it's not dijkstra!, dijkstra could add the same cell
        // multiple times to the heap)
        use std::cmp::Reverse;
        let mut min_heap =
            std::collections::BinaryHeap::from([(Reverse(grid[0][0]), 0usize, 0usize)]);
        let n = grid.len();
        let m = grid[0].len();
        let mut visited = vec![vec![false; m]; n];
        visited[0][0] = true;
        let mut results = vec![0; q_len];
        let mut points = 0;
        for query_idx in queries_indecies {
            let query = queries[query_idx];
            while let Some(&top) = min_heap.peek() {
                let (Reverse(cell_val), cell_x, cell_y) = top;
                if cell_val >= query {
                    break;
                }
                min_heap.pop();
                points += 1;
                for (di, dj) in [(0, 1), (1, 0), (-1i32, 0i32), (0, -1)] {
                    let ni = cell_x.wrapping_add(di as usize);
                    let nj = cell_y.wrapping_add(dj as usize);
                    if !(0..n).contains(&ni) || !(0..m).contains(&nj) || visited[ni][nj] {
                        continue;
                    }
                    visited[ni][nj] = true;
                    min_heap.push((Reverse(grid[ni][nj]), ni, nj));
                }
            }
            results[query_idx] = points;
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
        let queries = vec![5, 6, 2];
        let output = vec![5, 8, 1];
        let result = Solution::max_points(grid, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![5, 2, 1], vec![1, 1, 2]];
        let queries = vec![3];
        let output = vec![0];
        let result = Solution::max_points(grid, queries);
        assert_eq!(result, output);
    }
}
