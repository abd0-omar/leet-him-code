// https://leetcode.com/problems/count-the-number-of-fair-pairs/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        // start from 0
        // binary search on the first true, first >= lower
        // binary search on the last true, first <= upeer
        // the ans for the first idx is the range
        // 0, F, T, T, T, F
        nums.sort_unstable();
        let n = nums.len();
        let mut result = 0;
        for i in 0..n - 1 {
            let mut l = i + 1;
            let mut r = n - 1;
            let mut first_range: Option<usize> = None;

            while l <= r {
                let mid = l + (r - l) / 2;
                let target = nums[mid] + nums[i];
                // find first true
                if target >= lower && target <= upper {
                    first_range = Some(mid);
                    r = mid - 1;
                } else if target > upper {
                    // check overflow
                    if let (_, true) = mid.overflowing_sub(1) {
                        break;
                    }
                    r = mid - 1;
                } else if target < lower {
                    l = mid + 1;
                }
            }

            if first_range.is_none() {
                continue;
            }

            let mut l = i + 1;
            let mut r = n - 1;
            let mut last_range: Option<usize> = None;

            while l <= r {
                let mid = l + (r - l) / 2;
                let target = nums[mid] + nums[i];

                // find last true
                if target >= lower && target <= upper {
                    last_range = Some(mid);
                    l = mid + 1;
                } else if target > upper {
                    // check overflow
                    if let (_, true) = mid.overflowing_sub(1) {
                        break;
                    }
                    r = mid - 1;
                } else if target < lower {
                    l = mid + 1;
                }
            }

            // dbg!(i);
            // dbg!(first_range);
            // dbg!(last_range);
            // dbg!("---");

            // ans is range
            match (first_range, last_range) {
                (Some(f), Some(l)) => result += (l - f + 1) as i64,
                _ => (),
            }
            // dbg!(result);
        }
        // [0, 1, 2, 3, 4, 5]
        // [0, 1, 4, 4, 5, 7];

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        let output = 6;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        let output = 1;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, output);
    }
}
