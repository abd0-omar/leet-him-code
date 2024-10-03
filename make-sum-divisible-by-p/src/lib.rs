// pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
//     // brute-force
//     // the brute-force got tle
//     let n = nums.len();
//     let mut prefix_sum = vec![0; n + 1];
//     for i in 1..n + 1 {
//         prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1] as i64;
//     }
//
//     dbg!(&prefix_sum);
//     // 0, 3, 4, 8, 10
//
//     let mut min: Option<i32> = None;
//
//     // check if nums array is alread divisible by p
//     if nums.iter().map(|&x| x as i64).sum::<i64>() % p as i64 == 0 {
//         return 0;
//     }
//
//     for i in 1..n + 1 {
//         for j in i..n + 1 {
//             // sum before of i
//             // sum after of j
//             let sum_before = prefix_sum[i - 1];
//             let sum_after = prefix_sum[n] - prefix_sum[j];
//             dbg!(sum_before);
//             dbg!(sum_after);
//             let total_sum = sum_before + sum_after;
//             if total_sum % p as i64 == 0 {
//                 if let Some(m) = min {
//                     min = Some(m.min((j - i + 1) as i32));
//                 } else {
//                     min = Some((j - i + 1) as i32)
//                 }
//             }
//         }
//     }
//     let res = min.unwrap_or(-1);
//     if res == n as i32 {
//         -1
//     } else {
//         res
//     }
// }

use std::collections::HashMap;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let total_sum = nums.iter().map(|&x| x as i64).sum::<i64>();
    let remainder = total_sum % p as i64;
    if remainder == 0 {
        return 0;
    }

    let mut prefix_sum = 0;
    let mut min_len = nums.len() as i32;
    let mut mod_map = HashMap::new();
    mod_map.insert(0, -1); // to handle the case when prefix_sum itself is divisible

    for (i, &num) in nums.iter().enumerate() {
        prefix_sum = (prefix_sum + num as i64) % p as i64;
        dbg!(&prefix_sum);
        dbg!(&remainder);
        let target_mod = (prefix_sum - remainder + p as i64) % p as i64;
        dbg!(&target_mod);

        if let Some(&prev_index) = mod_map.get(&target_mod) {
            dbg!("min");
            dbg!(&mod_map);
            min_len = min_len.min(i as i32 - prev_index);
        }

        // Store the mod result with the index
        mod_map.insert(prefix_sum, i as i32);
        dbg!("map");
        dbg!(&mod_map);
    }

    if min_len == nums.len() as i32 {
        -1
    } else {
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        let output = 1;
        let result = min_subarray(nums, p);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1000000000, 1000000000, 1000000000];
        let p = 3;
        let output = 0;
        let result = min_subarray(nums, p);
        assert_eq!(result, output);
    }
}
