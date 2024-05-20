use std::cmp::Reverse;

#[derive(Ord, PartialEq, PartialOrd, Eq)]
struct ValAndCord {
    val: Reverse<i32>,
    i: usize,
    j: usize,
}

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(ValAndCord {
        val: Reverse(grid[0][0]),
        i: 0,
        j: 0,
    });

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut max_depth = 0;

    while let Some(ValAndCord {
        val: cur_val,
        i: cur_i,
        j: cur_j,
    }) = pq.pop()
    {
        let cur_val = cur_val.0;
        println!("cur_val={:?}", cur_val);
        max_depth = max_depth.max(cur_val);
        visited[cur_i][cur_j] = true;
        if cur_i == grid.len() - 1 && cur_j == grid[0].len() - 1 {
            break;
        }

        for (di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let ni = (cur_i as i32 + di) as usize;
            let nj = (cur_j as i32 + dj) as usize;
            if (0..grid.len()).contains(&ni) && (0..grid[0].len()).contains(&nj) && !visited[ni][nj]
            {
                let neighbor_val = grid[ni][nj];
                pq.push(ValAndCord {
                    val: Reverse(neighbor_val),
                    i: ni,
                    j: nj,
                })
            }
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        let output = 16;
        let result = swim_in_water(grid);
        assert_eq!(result, output);
    }
}
