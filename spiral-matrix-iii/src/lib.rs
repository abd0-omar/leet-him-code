pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    // right += 1;
    // down += 1;
    // left += 2;
    // up += 2;
    // every two iterations
    let mut steps = 1;
    let mut result = Vec::new();

    // right, down, up, left
    let mut directions = [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().cycle();

    let mut r = r_start;
    let mut c = c_start;

    while (result.len() as i32) < rows * cols {
        for _ in 0..2 {
            let (dr, dc) = directions.next().unwrap();

            for _ in 0..steps {
                if (0..rows).contains(&r) && (0..cols).contains(&c) {
                    result.push(vec![r, c]);
                }

                r = r + dr;
                c = c + dc;
            }
        }
        steps += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rows = 5;
        let cols = 6;
        let r_start = 1;
        let c_start = 4;
        let output = vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
        ];
        let result = spiral_matrix_iii(rows, cols, r_start, c_start);
        assert_eq!(result, output);
    }
}
