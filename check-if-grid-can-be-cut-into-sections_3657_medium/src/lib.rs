// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut all_x: Vec<(i32, i32)> = rectangles.iter().map(|x| (x[0], x[2])).collect();
        let mut all_y: Vec<(i32, i32)> = rectangles.iter().map(|y| (y[1], y[3])).collect();
        all_x.sort_unstable();
        all_y.sort_unstable();

        let x_merged = vec![all_x[0]];
        let y_merged = vec![all_y[0]];

        if cut_sections(all_y, y_merged, 0) || cut_sections(all_x, x_merged, 0) {
            return true;
        }

        false
    }
}

fn cut_sections(all: Vec<(i32, i32)>, mut merged: Vec<(i32, i32)>, mut result: i32) -> bool {
    for (cur_start, cur_end) in all.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if last.1 > cur_start {
            // merge needed for last
            last.1 = last.1.max(cur_end);
        } else {
            result += 1;
            if result == 2 {
                return true;
            }
            // a new merge added to the part
            merged.push((cur_start, cur_end));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 5;
        let rectangles = vec![
            vec![1, 0, 5, 2],
            vec![0, 2, 2, 4],
            vec![3, 2, 5, 3],
            vec![0, 4, 4, 5],
        ];
        let output = true;
        let result = Solution::check_valid_cuts(n, rectangles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 4;
        let rectangles = vec![
            vec![0, 0, 1, 1],
            vec![2, 0, 3, 4],
            vec![0, 2, 2, 3],
            vec![3, 0, 4, 3],
        ];
        let output = true;
        let result = Solution::check_valid_cuts(n, rectangles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 4;
        let rectangles = vec![
            vec![0, 2, 2, 4],
            vec![1, 0, 3, 2],
            vec![2, 2, 3, 4],
            vec![3, 0, 4, 2],
            vec![3, 2, 4, 4],
        ];
        let output = false;
        let result = Solution::check_valid_cuts(n, rectangles);
        assert_eq!(result, output);
    }
}
