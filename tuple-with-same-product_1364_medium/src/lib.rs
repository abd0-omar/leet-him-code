// https://leetcode.com/problems/tuple-with-same-product/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        // input is too low (1000)
        // O(n ^ 2) calculate frequency of each product
        let n = nums.len();
        let mut result = 0;
        let mut freq = std::collections::HashMap::new();
        for i in 0..n - 1 {
            let first = nums[i];
            for j in i + 1..n {
                *freq.entry(first * nums[j]).or_insert(0) += 1;
            }
        }
        dbg!(&freq);
        for count in freq.into_values() {
            if count >= 2 {
                result += count * (count - 1) * 4;
            }
            dbg!(count);
            dbg!(result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 3, 4, 6];
        let output = 8;
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 4, 5, 10];
        let output = 16;
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let nums = vec![2, 3, 4, 6, 8, 12];
        let output = 40;
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, output);
    }
}
