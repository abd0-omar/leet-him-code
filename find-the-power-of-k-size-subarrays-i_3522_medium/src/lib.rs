// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // input is too small, a brute force could work
        // and it's optimal solution is still a brute force "fixed sliding window"
        // but I could optimize getting the max of the window without recalculating

        // brute-force
        let n = nums.len();
        let k = k as usize;
        //                                 one_based - one_based
        //                                 so it'll turn to 0 based, add one to return to 1 based
        let mut result = vec![-1; n - k + 1];

        let mut l = 0;
        let mut r = 0;
        let mut idx = 0;
        while r < n {
            if r < k - 1 {
                r += 1;
            } else {
                // see if the window is sorted
                let mut is_consecutive = true;
                for j in l + 1..=r {
                    if nums[j] != nums[j - 1] + 1 {
                        is_consecutive = false;
                        break;
                    }
                }
                // get the max
                if is_consecutive {
                    let mut max = nums[l];
                    for j in l + 1..=r {
                        max = max.max(nums[j]);
                    }
                    result[idx] = max;
                }

                idx += 1;
                r += 1;
                l += 1;
            }
        }

        // this passed, but in an interview I don't think I would've passed
        // max fn is redundant as the max element is always at the end
        // to check if it is consecutive I could've used a simple counter without the extra for loop

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 3, 4, 3, 2, 5];
        let k = 3;
        let output = vec![3, 4, -1, -1, -1];
        let result = Solution::results_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 2, 2, 2, 2];
        let k = 4;
        let output = vec![-1, -1];
        let result = Solution::results_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 2, 3, 2, 3, 2];
        let k = 2;
        let output = vec![-1, 3, -1, 3, -1];
        let result = Solution::results_array(nums, k);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works3() {
        let nums = vec![1, 3, 4];
        let k = 2;
        let output = vec![-1, 4];
        let result = Solution::results_array(nums, k);
        assert_eq!(result, output);
    }
}
