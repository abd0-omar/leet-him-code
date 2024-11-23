// https://leetcode.com/problems/rotating-the-box/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_the_box(mybox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = mybox.len();
        let n = mybox[0].len();
        let mut rotated = vec![vec!['.'; m]; n];

        // Rotate the box 90 degrees clockwise
        for (row_idx, row) in mybox.iter().enumerate() {
            let mut empty_row = n;
            for (col_idx, &cell) in row.iter().enumerate().rev() {
                match &cell {
                    '#' => {
                        empty_row -= 1;
                        rotated[empty_row][m - row_idx - 1] = cell;
                    }
                    '*' => {
                        rotated[col_idx][m - row_idx - 1] = '*';
                        empty_row = col_idx;
                    }
                    _ => (),
                }
            }
        }

        rotated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let my_box = vec![vec!['#', '.', '#']];
        let output = vec![vec!['.'], vec!['#'], vec!['#']];
        let result = Solution::rotate_the_box(my_box);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let my_box = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let output = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        let result = Solution::rotate_the_box(my_box);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let my_box = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let output = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];
        let result = Solution::rotate_the_box(my_box);
        assert_eq!(result, output);
    }
}
