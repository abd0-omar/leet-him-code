// https://leetcode.com/problems/making-a-large-island/
#[allow(dead_code)]
struct Solution;

// naijaboor
const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(dead_code)]
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        // simple hard problem, just takes a lot to code, but the core "bfs" idea is simple
        // find largest 1 forest
        // if there is no 0s then the answer is the largest 1s forest
        // otherwise the cur_max_result is the largest forest + 1 (if we flipped one 0)
        // for each 0, find if we can connect two islands
        // in the beginning of the code, map each 1 cell to a group, and map the group it's size.
        // that's it
        let n = grid.len();
        let m = grid[0].len();
        // (i, j) | group_number
        let mut group = HashMap::new();
        let mut group_number = 0;

        // could've marked each cell of the grid with it's group size, but we don't want to change
        // the original input, working professionally, if I really wanted to work professionally I
        // would've made `group` and `group_size` in a struct, but ورانا حاجات تانية
        // group_number | group_size
        let mut group_size = HashMap::new();

        let mut is_there_zero = false;

        let mut visited = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && !visited[i][j] {
                    let cur_group_size =
                        bfs(&grid, i, j, &mut visited, n, m, &mut group, group_number);
                    group_size.insert(group_number, cur_group_size);
                    group_number += 1;
                }

                if grid[i][j] == 0 {
                    is_there_zero = true;
                }
            }
        }

        let largest_forest_group_size = *group_size.values().max().unwrap_or(&0);

        if !is_there_zero {
            return largest_forest_group_size;
        }

        let mut result = largest_forest_group_size + 1;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    let mut unique_groups = HashSet::with_capacity(4);
                    // flip zero
                    let mut cur_merged_result = 1;
                    for (di, dj) in DIR {
                        let ni = i.wrapping_add(di as usize);
                        let nj = j.wrapping_add(dj as usize);

                        if (0..n).contains(&ni) && (0..m).contains(&nj) && grid[ni][nj] == 1 {
                            if let Some(cur_group_number) = group.get(&(ni, nj)) {
                                if let Some(&cur_group_size) = group_size.get(&cur_group_number) {
                                    if unique_groups.insert(cur_group_number) {
                                        cur_merged_result += cur_group_size;
                                    }
                                }
                            }
                        }

                        result = result.max(cur_merged_result);
                    }
                }
            }
        }

        result
    }
}

fn bfs(
    grid: &[Vec<i32>],
    i: usize,
    j: usize,
    visited: &mut [Vec<bool>],
    n: usize,
    m: usize,
    group: &mut HashMap<(usize, usize), i32>,
    group_number: i32,
) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((i, j));
    visited[i][j] = true;
    group.insert((i, j), group_number);
    let mut grid_size = 1;

    while let Some((i, j)) = q.pop_front() {
        for (di, dj) in DIR {
            let ni = i.wrapping_add(di as usize);
            let nj = j.wrapping_add(dj as usize);
            if (0..n).contains(&ni) && (0..m).contains(&nj) && grid[ni][nj] == 1 && !visited[ni][nj]
            {
                q.push_back((ni, nj));
                visited[ni][nj] = true;
                group.insert((ni, nj), group_number);
                grid_size += 1;
            }
        }
    }

    grid_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let output = 3;
        let result = Solution::largest_island(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 1], vec![1, 0]];
        let output = 4;
        let result = Solution::largest_island(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        let output = 4;
        let result = Solution::largest_island(grid);
        assert_eq!(result, output);
    }
}
