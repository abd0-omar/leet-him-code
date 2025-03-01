// https://leetcode.com/problems/apply-operations-to-an-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        /*
        nums[i] == nums[i + 1]
        nums[i] * 2, nums[i + 1] = 0

        shift 0's to the end
        */
        let n = nums.len();
        let mut i = 0;
        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
                i += 2;
                continue;
            }
            i += 1;
        }

        // shift zeros to the end
        let mut last_found_zero = 0;
        for i in 0..n {
            if nums[i] != 0 {
                nums[last_found_zero] = nums[i];
                last_found_zero += 1;
            }
        }

        while last_found_zero < n {
            nums[last_found_zero] = 0;
            last_found_zero += 1;
        }

        // not space optimized
        // let mut result: Vec<i32> = nums.into_iter().filter(|&num| num != 0).collect();
        // let mut zero_count = n - result.len();

        // while zero_count > 0 {
        //     result.push(0);
        //     zero_count -= 1;
        // }

        // result
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 2, 1, 1, 0];
        let output = vec![1, 4, 2, 0, 0, 0];
        let result = Solution::apply_operations(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![0, 1];
        let output = vec![1, 0];
        let result = Solution::apply_operations(nums);
        assert_eq!(result, output);
    }
}
