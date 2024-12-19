// https://leetcode.com/problems/max-chunks-to-make-sorted/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        // the monotonic stack solution is somewhat good to if you want to check it out future me
        let mut result = 0;
        let mut cur_max = -1;
        for (idx, num) in arr.into_iter().enumerate() {
            cur_max = cur_max.max(num);
            if cur_max == idx as i32 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![4, 3, 2, 1, 0];
        let output = 1;
        let result = Solution::max_chunks_to_sorted(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 0, 2, 3, 4];
        let output = 4;
        let result = Solution::max_chunks_to_sorted(arr);
        assert_eq!(result, output);
    }
}
