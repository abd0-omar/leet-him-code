// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/
#[allow(dead_code)]
struct Solution;

#[derive(Eq, PartialEq)]
struct CellAndObstacles {
    i: usize,
    j: usize,
    obstacles: i32,
}

impl Ord for CellAndObstacles {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.obstacles.cmp(&self.obstacles)
    }
}

impl PartialOrd for CellAndObstacles {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CellAndObstacles {
    fn new(i: usize, j: usize, obstacles: i32) -> Self {
        Self { i, j, obstacles }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        // dijkstra and count the obstacles that you'll hit
        // dfs seems intuitive by just counting
        let (n, m) = (grid.len(), grid[0].len());
        let mut pq = std::collections::BinaryHeap::from([CellAndObstacles::new(0, 0, 0)]);
        let mut obstacles = vec![vec![i32::MAX; m]; n];
        obstacles[0][0] = if grid[0][0] == 0 { 0 } else { 1 };

        while let Some(cur_cell) = pq.pop() {
            if cur_cell.i == n - 1 && cur_cell.j == m - 1 {
                return cur_cell.obstacles;
            }

            for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = cur_cell.i.wrapping_add(di as usize);
                let nj = cur_cell.j.wrapping_add(dj as usize);

                if (0..n).contains(&ni) && (0..m).contains(&nj) {
                    let is_obstacle = if grid[ni][nj] == 1 { 1 } else { 0 };
                    if obstacles[ni][nj] > cur_cell.obstacles + is_obstacle {
                        obstacles[ni][nj] = cur_cell.obstacles + is_obstacle;
                        pq.push(CellAndObstacles::new(
                            ni,
                            nj,
                            cur_cell.obstacles + is_obstacle,
                        ));
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
        let grid = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let output = 2;
        let result = Solution::minimum_obstacles(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        let output = 0;
        let result = Solution::minimum_obstacles(grid);
        assert_eq!(result, output);
    }
}
