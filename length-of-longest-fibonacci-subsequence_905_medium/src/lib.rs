// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
#[allow(dead_code)]
struct Solution;

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let arr_map = arr
            .iter()
            .enumerate()
            .map(|(idx, &num)| (num, idx))
            .collect::<HashMap<i32, usize>>();
        let n = arr.len();
        let mut memory = vec![vec![0; n]; n];
        let mut max_len = 0;

        for i in 0..n - 1 {
            for j in i + 1..n {
                max_len = max_len.max(dp(i, j, &arr, &arr_map, &mut memory));
            }
        }

        if max_len >= 3 {
            max_len as i32
        } else {
            0
        }
    }
}

fn dp(
    i: usize,
    j: usize,
    arr: &[i32],
    arr_map: &HashMap<i32, usize>,
    memory: &mut Vec<Vec<usize>>,
) -> usize {
    if memory[i][j] != 0 {
        return memory[i][j];
    }

    let target = arr[i] + arr[j];
    if let Some(&k) = arr_map.get(&target) {
        let fibo_len = dp(j, k, arr, arr_map, memory) + 1;
        memory[i][j] = fibo_len;
    }

    memory[i][j] = memory[i][j].max(2);
    memory[i][j]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let output = 5;
        let result = Solution::len_longest_fib_subseq(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 3, 7, 11, 12, 14, 18];
        let output = 3;
        let result = Solution::len_longest_fib_subseq(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let arr = vec![2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50];
        let output = 5;
        let result = Solution::len_longest_fib_subseq(arr);
        assert_eq!(result, output);
    }
}
