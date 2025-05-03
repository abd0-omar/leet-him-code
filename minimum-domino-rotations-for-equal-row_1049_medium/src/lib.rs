// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut count_top = vec![0; 7];
        let mut count_bottom = vec![0i32; 7];
        let mut count_same = vec![0; 7];
        let n = tops.len();
        for i in 0..n {
            count_top[tops[i] as usize] += 1;
            count_bottom[bottoms[i] as usize] += 1;
            if tops[i] == bottoms[i] {
                count_same[tops[i] as usize] += 1;
            }
        }
        for i in 1..7 {
            if count_top[i] + count_bottom[i] - count_same[i] == n as i32 {
                return n as i32 - count_top[i].max(count_bottom[i]);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        let output = 2;
        let result = Solution::min_domino_rotations(tops, bottoms);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        let output = -1;
        let result = Solution::min_domino_rotations(tops, bottoms);
        assert_eq!(result, output);
    }
}
