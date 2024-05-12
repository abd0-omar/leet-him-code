pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut result = vec![vec![0; n - 2]; n - 2];
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            let mut local_max = 0;
            for k in i - 1..i + 2 {
                for l in j - 1..j + 2 {
                    if grid[k][l] > local_max {
                        local_max = grid[k][l];
                    }
                }
            }
            result[i - 1][j - 1] = local_max;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2],
        ];
        let output = vec![vec![9, 9], vec![8, 6]];
        let result = largest_local(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![20, 8, 20, 6, 16, 16, 7, 16, 8, 10],
            vec![12, 15, 13, 10, 20, 9, 6, 18, 17, 6],
            vec![12, 4, 10, 13, 20, 11, 15, 5, 17, 1],
            vec![7, 10, 14, 14, 16, 5, 1, 7, 3, 11],
            vec![16, 2, 9, 15, 9, 8, 6, 1, 7, 15],
            vec![18, 15, 18, 8, 12, 17, 19, 7, 7, 8],
            vec![19, 11, 15, 16, 1, 3, 7, 4, 7, 11],
            vec![11, 6, 5, 14, 12, 18, 3, 20, 14, 6],
            vec![4, 4, 19, 6, 17, 12, 8, 8, 18, 8],
            vec![19, 15, 14, 11, 11, 13, 12, 6, 16, 19],
        ];
        let output = vec![
            vec![20, 20, 20, 20, 20, 18, 18, 18],
            vec![15, 15, 20, 20, 20, 18, 18, 18],
            vec![16, 15, 20, 20, 20, 15, 17, 17],
            vec![18, 18, 18, 17, 19, 19, 19, 15],
            vec![19, 18, 18, 17, 19, 19, 19, 15],
            vec![19, 18, 18, 18, 19, 20, 20, 20],
            vec![19, 19, 19, 18, 18, 20, 20, 20],
            vec![19, 19, 19, 18, 18, 20, 20, 20],
        ];
        let result = largest_local(grid);
        assert_eq!(result, output);
    }
}
