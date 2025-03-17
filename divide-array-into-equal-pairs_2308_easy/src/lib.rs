// https://leetcode.com/problems/divide-array-into-equal-pairs/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut freq = vec![0; *nums.iter().max().unwrap() as usize + 1];
        for &num in &nums {
            freq[num as usize] += 1;
        }
        for num in nums {
            if freq[num as usize] % 2 != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 2, 3, 2, 2, 2];
        let output = true;
        let result = Solution::divide_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3, 4];
        let output = false;
        let result = Solution::divide_array(nums);
        assert_eq!(result, output);
    }
}
