// https://leetcode.com/problems/find-if-array-can-be-sorted/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        // input is small so I can get away with O(n^2) solution
        let n = nums.len();

        // check if sorted
        for i in 1..n {
            if nums[i] < nums[i - 1] {
                break;
            }
            if i == n - 1 {
                return true;
            }
        }

        for _ in 1..n {
            for j in 1..n {
                if nums[j] < nums[j - 1] && nums[j].count_ones() == nums[j - 1].count_ones() {
                    nums.swap(j, j - 1);
                }
            }
        }

        // check if sorted
        for i in 1..n {
            if nums[i] < nums[i - 1] {
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
        let nums = vec![8, 4, 2, 30, 15];
        let output = true;
        let result = Solution::can_sort_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3, 4, 5];
        let output = true;
        let result = Solution::can_sort_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 16, 8, 4, 2];
        let output = false;
        let result = Solution::can_sort_array(nums);
        assert_eq!(result, output);
    }
}
