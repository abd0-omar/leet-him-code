// https://leetcode.com/problems/map-of-highest-peak/
#[allow(dead_code)]
struct Solution;

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

#[allow(dead_code)]
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // multi source bfs
        let mut q = std::collections::VecDeque::new();
        let n = is_water.len();
        let m = is_water[0].len();
        let mut visited = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    q.push_back((i, j));
                    visited[i][j] = true;
                }
            }
        }
        let mut lvl = 0;
        let mut result_matrix = vec![vec![0; m]; n];

        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (i, j) = q.pop_front().unwrap();
                result_matrix[i][j] = lvl;

                dbg!(&result_matrix);
                for (di, dj) in DIR {
                    // let ni = (i as i32 + di) as usize;
                    // let nj = (j as i32 + dj) as usize;
                    let ni = i.wrapping_add(di as usize);
                    let nj = j.wrapping_add(dj as usize);

                    if (0..n).contains(&ni) && (0..m).contains(&nj) && !visited[ni][nj] {
                        q.push_back((ni, nj));
                        visited[ni][nj] = true;
                    }
                }
            }
            lvl += 1;
        }
        result_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let is_water = vec![vec![0, 1], vec![0, 0]];
        let output = vec![vec![1, 0], vec![2, 1]];
        let result = Solution::highest_peak(is_water);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        let output = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]];
        let result = Solution::highest_peak(is_water);
        assert_eq!(result, output);
    }
}
