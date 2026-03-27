// https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_similar(mut mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat.len();
        let m = mat[0].len();
        let k = k as usize % m;

        let original = mat.clone();

        for _ in 0..k {
            for i in 0..n {
                if i % 2 == 0 {
                    shift_left(&mut mat, i);
                } else {
                    shift_right(&mut mat, i);
                }
            }
        }

        mat == original
    }
}

fn shift_left(mat: &mut Vec<Vec<i32>>, i: usize) {
    let row = &mat[i];
    let first_cell = row[0];
    let mut new_row = vec![0; row.len()];
    for idx in 1..row.len() {
        new_row[idx - 1] = row[idx];
    }
    new_row[row.len() - 1] = first_cell;
    mat[i] = new_row;
}

fn shift_right(mat: &mut Vec<Vec<i32>>, i: usize) {
    let row = &mat[i];
    let last_cell = row[row.len() - 1];
    let mut new_row = vec![0; row.len()];
    for idx in 1..row.len() {
        new_row[idx] = row[idx - 1];
    }
    new_row[0] = last_cell;
    mat[i] = new_row;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 4;
        let output = false;
        let result = Solution::are_similar(mat, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let k = 2;
        let output = true;
        let result = Solution::are_similar(mat, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let mat = vec![vec![2, 2], vec![2, 2]];
        let k = 3;
        let output = true;
        let result = Solution::are_similar(mat, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let mat = vec![
            vec![4, 9, 10, 10],
            vec![9, 3, 8, 4],
            vec![2, 5, 3, 8],
            vec![6, 1, 10, 4],
        ];
        let k = 5;
        let output = false;
        let result = Solution::are_similar(mat, k);
        assert_eq!(result, output);
    }
}
