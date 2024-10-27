pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    // it's like "maximal square" leetcode problem
    // and I guess it could work with binary search too
    // but would go with the dp route

    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = 0;
    let mut memo = vec![vec![-1; m]; n];

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 1 {
                let dp = dp(&matrix, i, j, &mut memo);
                result += dp;
                dbg!((i, j));
                dbg!(dp);
            }
        }
    }
    result
}

// input is low so we can get a way with a vec memo
fn dp(matrix: &Vec<Vec<i32>>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if !(0..matrix.len()).contains(&i) || !(0..matrix[0].len()).contains(&j) || matrix[i][j] == 0 {
        return 0;
    }

    let ret = memo[i][j];
    if ret != -1 {
        return ret;
    }

    // 1 1        1 1 1
    // 1 2   =>   1 2 2
    //            1 2 3
    //
    // we check the previous 3 neighbor squares and get the min

    let count_squares = if matrix[i][j] == 1 {
        dp(matrix, i + 1, j + 1, memo)
            .min(dp(matrix, i + 1, j, memo))
            .min(dp(matrix, i, j + 1, memo))
            + 1
    } else {
        0
    };

    memo[i][j] = count_squares;
    count_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        let output = 15;
        let result = count_squares(matrix);
        assert_eq!(result, output);
    }
}
