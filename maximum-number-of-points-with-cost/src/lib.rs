use std::collections::HashMap;
pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
    // I think it's backtracking (dp), I don't think or can't think of a greedy way
    let mut max = 0;

    let mut memo = HashMap::new();

    for col in 0..points[0].len() {
        let cur_max = _max_points(&points, 0, col, &mut memo);
        dbg!(cur_max);
        max = max.max(cur_max);
    }
    max
}

fn _max_points(
    points: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if row == points.len() {
        return 0;
    }

    if let Some(&ret) = memo.get(&(row, col)) {
        return ret;
    }

    let mut max = 0;

    for nxt_col in 0..points[0].len() {
        max = max.max(
            _max_points(points, row + 1, nxt_col, memo) + points[row][col as usize] as i64
                - (nxt_col as i64 - col as i64).abs(),
        );
    }

    memo.insert((row, col), max);

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let points = vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]];
        let output = 9;
        let result = max_points(points);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let points = vec![vec![1, 5], vec![2, 3], vec![4, 2]];
        let output = 11;
        let result = max_points(points);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let points = vec![vec![1], vec![2], vec![3]];
        let output = 6;
        let result = max_points(points);
        assert_eq!(result, output);
    }
}
