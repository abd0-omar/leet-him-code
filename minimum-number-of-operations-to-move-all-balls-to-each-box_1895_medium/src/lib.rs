// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        // input too low that could be brute-forced
        // and can be slightly optimized with a hashmap
        // but maintaining the sum of ones on the left and on the right is trivial too
        //  "001011"
        //  [0,0,2,0,6,11]
        //  idx 0: 11 => 11 - 0 * 3
        //  idx 1:  8 => 11 - 1 * 3

        //  we landed on a 1
        //  11 - 2 => 9
        //  idx 2:  5 => 9 - 2 * 2

        //  we passed a 1
        //  left_counter = 1
        //  idx 3:  4 => 9 - 3 * 2 + 1

        //  we landed on a 1
        //  left_counter = 2
        //  9 - 4 => 5
        //  (9 - 4)
        //  idx 4:  3 => 5 - 4 * 1 + 2

        //  we landed on a 1
        //  left_counter = 3
        //  5 - 5 => 0
        //  double the left counter val because we have two ones on the left
        //  left_counter = 4
        //  if 0 then the answer is the left_counter
        //  idx 5:  4 => 4
        //
        // this comments are not entirely correct

        let mut left_ball_count = 0;
        let mut right_ball_count = 0;
        let mut left_counter = 0;
        let mut right_counter = 0;

        for (idx, ch) in boxes.chars().enumerate() {
            if ch == '1' {
                right_ball_count += 1;
                right_counter += idx;
            }
        }

        let mut result = vec![0; boxes.len()];
        for (idx, ch) in boxes.chars().enumerate() {
            result[idx] = (left_counter + right_counter) as i32;

            if ch == '1' {
                left_ball_count += 1;
                right_ball_count -= 1;
            }

            left_counter += left_ball_count;
            right_counter -= right_ball_count;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let boxes = "110".to_string();
        let output = vec![1, 1, 3];
        let result = Solution::min_operations(boxes);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let boxes = "001011".to_string();
        let output = vec![11, 8, 5, 4, 3, 4];
        let result = Solution::min_operations(boxes);
        assert_eq!(result, output);
    }
}
