// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        // it's a permutation, so every number will appear once in each array
        // so maintain one freq array for both
        // if freq == 2 then count++, result[i] = count
        // input is too low too, so freq array would be sufficient instead of a hashmap
        let n = a.len();
        let mut freq = vec![0; n + 1];
        let mut count = 0;
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            freq[a[i] as usize] += 1;
            freq[b[i] as usize] += 1;

            if a[i] == b[i] {
                count += 1;
            } else {
                if freq[a[i] as usize] == 2 {
                    count += 1;
                }
                if freq[b[i] as usize] == 2 {
                    count += 1;
                }
            }

            result.push(count);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let a = vec![1, 3, 2, 4];
        let b = vec![3, 1, 2, 4];
        let output = vec![0, 2, 3, 4];
        let result = Solution::find_the_prefix_common_array(a, b);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let a = vec![2, 3, 1];
        let b = vec![3, 1, 2];
        let output = vec![0, 1, 3];
        let result = Solution::find_the_prefix_common_array(a, b);
        assert_eq!(result, output);
    }
}
