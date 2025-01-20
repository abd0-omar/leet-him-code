#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        let mut value_to_pos = std::collections::HashMap::new();
        for i in 0..n {
            for j in 0..m {
                value_to_pos.insert(mat[i][j], (i, j));
            }
        }
        dbg!(&value_to_pos);

        let mut row_freq = vec![0; n];
        let mut col_freq = vec![0; m];

        for (index, &value) in arr.iter().enumerate() {
            let (row, col) = value_to_pos[&value];
            row_freq[row] += 1;
            col_freq[col] += 1;

            if row_freq[row] == m || col_freq[col] == n {
                return index as i32;
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
        let arr = vec![1, 3, 4, 2];
        let mat = vec![vec![1, 4], vec![2, 3]];
        let output = 2;
        let result = Solution::first_complete_index(arr, mat);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        let output = 3;
        let result = Solution::first_complete_index(arr, mat);
        assert_eq!(result, output);
    }
}
