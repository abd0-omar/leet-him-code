pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    let mut pq = std::collections::BinaryHeap::new();
    let m = grid.len();
    let n = grid[0].len();
    pq.push((Reverse(grid[0][0]), (0, 0)));

    let mut distance = vec![vec![i32::MAX; n]; m];
    distance[0][0] = grid[0][0];

    while let Some((Reverse(val), (i, j))) = pq.pop() {
        if i == m - 1 && j == n - 1 {
            return val;
        }

        for (di, dj) in [(0, 1), (1, 0)] {
            let ni = (di as i32 + i as i32) as usize;
            let nj = (dj as i32 + j as i32) as usize;
            if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
                continue;
            }

            let new_cost = val + grid[ni][nj];
            if distance[ni][nj] == i32::MAX || new_cost < distance[ni][nj] {
                distance[ni][nj] = new_cost;
                pq.push((Reverse(new_cost), (ni, nj)));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let output = 7;
        let result = min_path_sum(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![5, 4, 2, 9, 6, 0, 3, 5, 1, 4, 9, 8, 4, 9, 7, 5, 1],
            vec![3, 4, 9, 2, 9, 9, 0, 9, 7, 9, 4, 7, 8, 4, 4, 5, 8],
            vec![6, 1, 8, 9, 8, 0, 3, 7, 0, 9, 8, 7, 4, 9, 2, 0, 1],
            vec![4, 0, 0, 5, 1, 7, 4, 7, 6, 4, 1, 0, 1, 0, 6, 2, 8],
            vec![7, 2, 0, 2, 9, 3, 4, 7, 0, 8, 9, 5, 9, 0, 1, 1, 0],
            vec![8, 2, 9, 4, 9, 7, 9, 3, 7, 0, 3, 6, 5, 3, 5, 9, 6],
            vec![8, 9, 9, 2, 6, 1, 2, 5, 8, 3, 7, 0, 4, 9, 8, 8, 8],
            vec![5, 8, 5, 4, 1, 5, 6, 6, 3, 3, 1, 8, 3, 9, 6, 4, 8],
            vec![0, 2, 2, 3, 0, 2, 6, 7, 2, 3, 7, 3, 1, 5, 8, 1, 3],
            vec![4, 4, 0, 2, 0, 3, 8, 4, 1, 3, 3, 0, 7, 4, 2, 9, 8],
            vec![5, 9, 0, 4, 7, 5, 7, 6, 0, 8, 3, 0, 0, 6, 6, 6, 8],
            vec![0, 7, 1, 8, 3, 5, 1, 8, 7, 0, 2, 9, 2, 2, 7, 1, 5],
            vec![1, 0, 0, 0, 6, 2, 0, 0, 2, 2, 8, 0, 9, 7, 0, 8, 0],
            vec![1, 1, 7, 2, 9, 6, 5, 4, 8, 7, 8, 5, 0, 3, 8, 1, 5],
            vec![8, 9, 7, 8, 1, 1, 3, 0, 1, 2, 9, 4, 0, 1, 5, 3, 1],
            vec![9, 2, 7, 4, 8, 7, 3, 9, 2, 4, 2, 2, 7, 8, 2, 6, 7],
            vec![3, 8, 1, 6, 0, 4, 8, 9, 8, 0, 2, 5, 3, 5, 5, 7, 5],
            vec![1, 8, 2, 5, 7, 7, 1, 9, 9, 8, 9, 2, 4, 9, 5, 4, 0],
            vec![3, 4, 4, 1, 5, 3, 3, 8, 8, 6, 3, 5, 3, 8, 7, 1, 3],
        ];
        let output = 82;
        let result = min_path_sum(grid);
        assert_eq!(result, output);
    }
}
