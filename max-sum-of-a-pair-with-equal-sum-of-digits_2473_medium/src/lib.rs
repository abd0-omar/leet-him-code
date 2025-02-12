// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        // let nums = vec![18, 43, 36, 13, 7];
        /*
        sum (1 + 8) = 9, hm[9] = 0
        sum (4 + 3) = 7, hm[7] = 1
        sum (3 + 6) = 9,
        found hm[9] = 0, new_cur_max_sum = new_cur_max_sum.max(nums[0] + nums[cur_index])
        update hm[9] with the max digit pair, 18.max(36), so update hm[9] = 2
        */
        let mut hm = std::collections::HashMap::new();
        let mut result = -1;
        for (idx, &num) in nums.iter().enumerate() {
            let cur_sum_digits = sum_digits(num);
            let cur_num = num;
            use std::collections::hash_map::Entry;
            match hm.entry(cur_sum_digits) {
                Entry::Occupied(mut occupied_entry) => {
                    let old_sum_digits_idx = *occupied_entry.get();
                    let old_num = nums[old_sum_digits_idx];
                    // dbg!(idx);
                    // dbg!(&cur_sum_digits);
                    // dbg!(&old_num);
                    // compute result
                    result = result.max(old_num + cur_num);

                    // update hm if cur > old
                    if cur_num > old_num {
                        // update hm
                        occupied_entry.insert(idx);
                    }
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(idx);
                }
            }
        }
        result
    }
}

fn sum_digits(mut num: i32) -> i32 {
    // eg.: 11
    // 1, sum = 1
    // 0, sum = 2
    let mut sum_digits = 0;
    while num > 0 {
        let last = num % 10;
        sum_digits += last;
        num = num / 10;
    }
    sum_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![18, 43, 36, 13, 7];
        let output = 54;
        let result = Solution::maximum_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![10, 12, 19, 14];
        let output = -1;
        let result = Solution::maximum_sum(nums);
        assert_eq!(result, output);
    }
}
