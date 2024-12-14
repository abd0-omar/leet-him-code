// https://leetcode.com/problems/continuous-subarrays/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        // from the leetcode discussion:
        // “It took sometime to realize that,
        // In example 1, the reason why sub arrays [5,4,2] or [5,4,2,4] is not
        // included in the solution is due to the fact that nums[0] - nums[2]
        // exceeds 2. 5-2=3 which is greater than 2. taking any 2 index into
        // consideration the unsigned difference should be less than or equal to
        // 2.”
        let mut freq = std::collections::BTreeMap::new();
        let mut st = 0;
        let n = nums.len();
        let mut count = 0;
        for end in 0..n {
            *freq.entry(nums[end]).or_insert(0) += 1;
            while freq.last_key_value().unwrap().0 - freq.first_key_value().unwrap().0 > 2 {
                if freq[&nums[st]] > 1 {
                    *freq.get_mut(&nums[st]).unwrap() -= 1;
                } else {
                    freq.remove(&nums[st]);
                }
                st += 1;
            }
            count += (end - st + 1) as i64;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![5, 4, 2, 4];
        let output = 8;
        let result = Solution::continuous_subarrays(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3];
        let output = 6;
        let result = Solution::continuous_subarrays(nums);
        assert_eq!(result, output);
    }
}
