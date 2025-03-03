// https://leetcode.com/problems/partition-array-according-to-given-pivot/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut count_pivot = 0;

        // smaller than pivot
        for &num in nums.iter() {
            if num < pivot {
                result.push(num);
            } else if num == pivot {
                count_pivot += 1;
            }
        }

        // equal
        (0..count_pivot).for_each(|_| result.push(pivot));

        // bigger
        for &num in nums.iter() {
            if num > pivot {
                result.push(num);
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
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        let output = vec![9, 5, 3, 10, 10, 12, 14];
        let result = Solution::pivot_array(nums, pivot);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        let output = vec![-3, 2, 4, 3];
        let result = Solution::pivot_array(nums, pivot);
        assert_eq!(result, output);
    }
}
