// https://leetcode.com/problems/longest-nice-subarray/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (1, n);
        let mut ans = 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(&nums, mid, n) {
                ans = mid as i32;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans
    }
}

fn possible(nums: &[i32], candidate_length: usize, n: usize) -> bool {
    // dbg!(&candidate_length);
    'outer: for i in 0..n - candidate_length + 1 {
        let mut cur_bitwise = 0i64;
        // dbg!(i);
        for j in i..i + candidate_length {
            // dbg!(j);
            // dbg!(cur_bitwise);
            if nums[j] as i64 & cur_bitwise != 0 {
                continue 'outer;
            }
            cur_bitwise |= nums[j] as i64;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3, 8, 48, 10];
        let output = 3;
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![3, 1, 5, 11, 13];
        let output = 1;
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, output);
    }
}
