// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        // it can be solved using simple dfs, we will try that out.
        // To complicate it (for no good reason) we could use union-find
        // for a refresher on that topic, it was interesting in the editorial,
        // in the editorial he converted 2d array to 1d using `i * m + j`, and did the basic union-find
        // with extra `total_fishes` vector field, also `components size` is a better name than `ranks`
        // it could be solved using bfs, but we won't try that option
        // because some people say that bfs is short for boyfriendsearch
        //
        // DFS
        let n = grid.len();
        let m = grid[0].len();
        // or you could modify the original grid and update it with "0"
        let mut visited = vec![vec![false; m]; n];
        let mut max_fishes = 0;
        const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != 0 {
                    max_fishes = max_fishes.max(dfs(&grid, i, j, &mut visited, &DIR, n, m))
                }
            }
        }

        max_fishes
    }
}

fn dfs(
    grid: &[Vec<i32>],
    i: usize,
    j: usize,
    visited: &mut [Vec<bool>],
    dir: &[(i32, i32)],
    n: usize,
    m: usize,
) -> i32 {
    if !(0..n).contains(&i) || !(0..m).contains(&j) || visited[i][j] || grid[i][j] == 0 {
        return 0;
    }

    let mut count_fishes = 0;
    visited[i][j] = true;
    count_fishes += grid[i][j];

    for (di, dj) in dir {
        let ni = i.wrapping_add(*di as usize);
        let nj = j.wrapping_add(*dj as usize);
        count_fishes += dfs(grid, ni, nj, visited, dir, n, m);
    }

    count_fishes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ];
        let output = 7;
        let result = Solution::find_max_fish(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        let output = 1;
        let result = Solution::find_max_fish(grid);
        assert_eq!(result, output);
    }
}
