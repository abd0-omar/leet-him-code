struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let mut cnt = HashMap::from([(0, 1)]);
        let mut result = 0;
        let mut cur_nums_mod_modulo = 0;
        for i in 0..n {
            if nums[i] % modulo == k {
                cur_nums_mod_modulo += 1;
            }
            // magic
            result += *cnt
                .get(&((cur_nums_mod_modulo - k + modulo) % modulo))
                .unwrap_or(&0) as i64;
            *cnt.entry(cur_nums_mod_modulo % modulo).or_insert(0) += 1;
        }
        result
    }
}

use std::collections::HashMap;
