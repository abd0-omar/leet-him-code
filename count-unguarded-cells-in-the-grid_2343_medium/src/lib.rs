// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut graph = vec![vec![Cell::Empty; n]; m];
        for g in &guards {
            let (i, j) = (g[0] as usize, g[1] as usize);
            graph[i][j] = Cell::Guard;
        }
        for w in walls {
            let (i, j) = (w[0] as usize, w[1] as usize);
            graph[i][j] = Cell::Wall;
        }
        for g in guards {
            let (i, j) = (g[0] as usize, g[1] as usize);
            dbg!(i);
            dbg!(j);
            iterate(&mut graph, i, j, m, n);
        }
        let mut count_unguared = 0;
        dbg!(&graph);
        for row in graph {
            for cell in row {
                if cell == Cell::Empty {
                    count_unguared += 1;
                }
            }
        }
        count_unguared
    }
}

fn iterate(graph: &mut Vec<Vec<Cell>>, i: usize, j: usize, m: usize, n: usize) {
    // right
    for dj in 1..n {
        let nj = j + dj;
        if nj >= n || graph[i][nj] == Cell::Wall || graph[i][nj] == Cell::Guard {
            break;
        }
        graph[i][nj] = Cell::Guarded;
    }

    // left
    for dj in 1..=j {
        let nj = j - dj;
        if graph[i][nj] == Cell::Wall || graph[i][nj] == Cell::Guard {
            break;
        }
        graph[i][nj] = Cell::Guarded;
    }

    // down
    for di in 1..m {
        let ni = i + di;
        if ni >= m || graph[ni][j] == Cell::Wall || graph[ni][j] == Cell::Guard {
            break;
        }
        graph[ni][j] = Cell::Guarded;
    }

    // up
    for di in 1..=i {
        let ni = i - di;
        if graph[ni][j] == Cell::Wall || graph[ni][j] == Cell::Guard {
            break;
        }
        graph[ni][j] = Cell::Guarded;
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Wall,
    Guard,
    Empty,
    Guarded,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let m = 4;
        let n = 6;
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        let output = 7;
        let result = Solution::count_unguarded(m, n, guards, walls);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let m = 3;
        let n = 3;
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        let output = 4;
        let result = Solution::count_unguarded(m, n, guards, walls);
        assert_eq!(result, output);
    }
}
