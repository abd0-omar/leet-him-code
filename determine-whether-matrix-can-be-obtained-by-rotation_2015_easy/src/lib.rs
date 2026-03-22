// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        // brute-force rotate the mat 3 times

        if mat == target { return true; }

        for i in 1..=3 {
            let new_mat = rotate(mat.clone(), i);
            if new_mat == target {
                return true;
            }
        }

        false
    }
}

fn rotate(mut mat: Vec<Vec<i32>>, no_rotations: u8) -> Vec<Vec<i32>> {
    let n = mat.len();
    let mut result = vec![vec![0; n]; n];

    for _ in 0..no_rotations {
        for i in 0..n {
            for j in 0..n {
                result[i][j] = mat[j][n - i - 1]
            }
        }
        mat = result.clone();

        // first row
        // 0, 0 -> 0, 2
        // 0, 1 -> 1, 2
        // 0, 2 -> 2, 2
        // second rwo
        // 1, 0 -> 0, 1
        // 1, 1 -> 1, 1
        // 1, 2 -> 2, 1
        // third row
        // 2, 0 -> 0, 0
        // 2, 1 -> 1, 0
        // 2, 2 -> 2, 0
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let mat = vec![vec![0, 1], vec![1, 0]];
        let target = vec![vec![1, 0], vec![0, 1]];
        let output = true;
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let mat = vec![vec![0, 1], vec![1, 1]];
        let target = vec![vec![1, 0], vec![0, 1]];
        let output = false;
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let target = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
        let output = true;
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, output);
    }
}
