// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        // find the longest non decreasing prefix
        let n = arr.len();
        let mut left = 0;
        while left < n - 1 && arr[left] <= arr[left + 1] {
            left += 1;
        }

        // find the longest non increasing postfix from the end reverse
        let mut right = n - 1;
        while right > 0 && arr[right] >= arr[right - 1] {
            right -= 1;
        }

        // already sorted
        if left >= right {
            return 0;
        }

        let mut result = (n - left - 1).min(right) as i32;
        let mut i = 0;
        let mut j = right;
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                result = result.min((j - i - 1) as i32);
                i += 1;
            } else {
                j += 1;
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
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let output = 3;
        let result = Solution::find_length_of_shortest_subarray(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![5, 4, 3, 2, 1];
        let output = 4;
        let result = Solution::find_length_of_shortest_subarray(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let arr = vec![1, 2, 3];
        let output = 0;
        let result = Solution::find_length_of_shortest_subarray(arr);
        assert_eq!(result, output);
    }
}
