// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        // brute-force getting all sub arrays won't work (TLE)
        let mut cur_sum = 0;
        let mut even_count = 0;
        let mut odd_count = 0;
        let mut result = 0;
        let modo = 1_000_000_007;

        for num in arr {
            cur_sum += num;
            if cur_sum % 2 == 0 {
                result = (result + odd_count) % modo;
                even_count += 1;
            } else {
                result = (result + 1 + even_count) % modo;
                odd_count += 1;
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
        let arr = vec![1, 3, 5];
        let output = 4;
        let result = Solution::num_of_subarrays(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![2, 4, 6];
        let output = 0;
        let result = Solution::num_of_subarrays(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let output = 16;
        let result = Solution::num_of_subarrays(arr);
        assert_eq!(result, output);
    }
}
