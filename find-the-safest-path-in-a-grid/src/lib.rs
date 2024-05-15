pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut q = std::collections::VecDeque::new();
    let n = grid.len();

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                q.push_back((i, j));
            } else {
                grid[i][j] = -1;
            }
        }
    }

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let (i, j) = q.pop_front().unwrap();

            for &(di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = (di as i32 + i as i32) as usize;
                let nj = (dj as i32 + j as i32) as usize;

                let val = grid[i][j];

                if (0..n).contains(&ni) && (0..n).contains(&nj) && grid[ni][nj] == -1 {
                    grid[ni][nj] = val + 1;
                    q.push_back((ni, nj));
                }
            }
        }
    }

    // // binary search on the maximum distance from "1" that can reach the end
    // let mut low = 0;
    // let mut high = 0;
    //
    // for i in 0..n {
    //     for j in 0..n {
    //         high = high.max(grid[i][j]);
    //     }
    // }
    //
    // let mut answer = -1;
    //
    // while low <= high {
    //     let mid = low + (high - low) / 2;
    //     if can_reach_end(&grid, mid) {
    //         answer = mid;
    //         low = mid + 1;
    //     } else {
    //         high = mid - 1;
    //     }
    // }
    //
    // answer
    let mut pq = std::collections::BinaryHeap::new();
    pq.push((grid[0][0], 0, 0));
    grid[0][0] = -1;

    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if curr.1 == n - 1 && curr.2 == n - 1 {
            return curr.0;
        }

        for &(di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let ni = (di as i32 + curr.1 as i32) as usize;
            let nj = (dj as i32 + curr.2 as i32) as usize;
            if (0..n).contains(&ni) && (0..n).contains(&nj) && grid[ni][nj] != -1 {
                pq.push((curr.0.min(grid[ni][nj]), ni, nj));
                grid[ni][nj] = -1;
            }
        }
    }
    todo!()
}

fn can_reach_end(grid: &Vec<Vec<i32>>, min_safeness: i32) -> bool {
    let n = grid.len();
    if grid[0][0] < min_safeness || grid[n - 1][n - 1] < min_safeness {
        return false;
    }
    let mut visited = vec![vec![false; n]; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));
    visited[0][0] = true;

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let (i, j) = q.pop_front().unwrap();
            if i == n - 1 && j == n - 1 {
                return true;
            }

            for &(di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = (i as i32 + di as i32) as usize;
                let nj = (j as i32 + dj as i32) as usize;

                if (0..n).contains(&ni)
                    && (0..n).contains(&nj)
                    && !visited[ni][nj]
                    && grid[ni][nj] >= min_safeness
                {
                    visited[i][j] = true;
                    q.push_back((ni, nj));
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
        let output = 2;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let output = 0;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let output = 2;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let grid = vec![vec![0, 1, 1], vec![0, 1, 1], vec![1, 1, 1]];
        let output = 0;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let grid = vec![vec![0, 1, 1], vec![0, 1, 1], vec![0, 0, 0]];
        let output = 1;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works5() {
        let grid = vec![vec![1]];
        let output = 0;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works6() {
        let grid = vec![vec![0, 1, 1], vec![0, 0, 0], vec![0, 0, 0]];
        let output = 1;
        let result = maximum_safeness_factor(grid);
        assert_eq!(result, output);
    }
}
