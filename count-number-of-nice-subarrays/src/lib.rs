pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    //  [1, 1, 2, 1, 1]
    // (sum_odd, freq_odd)
    //[(0, 0), (1, 1), (2, 2), (2, 2), (3, 3), (4, 4)];
    //    i      j
    //    i              j
    //    i                      j
    // if freq_odd >= k {
    //    max_window = max_window.max(j - i);
    // }
    //
    // could do it in O(n^2)
    // O(n^2) worked because it was low input
    // let n = nums.len();
    // let mut prefx_sum_freq_odds = vec![(0, 0); n + 1];
    // for i in 1..n + 1 {
    //     // sum odd
    //     if nums[i - 1] % 2 == 1 {
    //         prefx_sum_freq_odds[i].0 = prefx_sum_freq_odds[i - 1].0 + nums[i - 1];
    //         prefx_sum_freq_odds[i].1 = prefx_sum_freq_odds[i - 1].1 + 1;
    //     } else {
    //         prefx_sum_freq_odds[i].0 = prefx_sum_freq_odds[i - 1].0;
    //         prefx_sum_freq_odds[i].1 = prefx_sum_freq_odds[i - 1].1;
    //     }
    // }

    // // dbg!(&prefx_sum_freq_odds);

    // let mut count = 0;
    // for i in 0..n + 1 {
    //     for j in i + 1..n + 1 {
    //         if prefx_sum_freq_odds[j].1 - prefx_sum_freq_odds[i].1 == k {
    //             count += 1;
    //         }
    //     }
    // }

    // count
    // hashmap O(n)
    let n = nums.len();
    let mut hm = std::collections::HashMap::new();
    //         idx   sum  freq
    // hm.insert(0, (0, 0));
    //        sum    idx, freq
    let mut sum = 0;
    //        sum    idx
    //          sum    freq_odd
    // final conclusion of hm
    //          sum    freq of how many times this sum appeared
    hm.insert(sum, 1);
    // our check is: do we have a previous sum with "kza"
    let mut count = 0;
    for i in 0..n {
        if nums[i] % 2 == 1 {
            sum += 1;
        }

        // we don't need the idx, we just count
        if let Some(f) = hm.get(&(sum - k)) {
            //      old count + freq of how many times that sum appeared up till now
            count = count + f;
        }

        hm.entry(sum).and_modify(|freq| *freq += 1).or_insert(1);
        dbg!(&hm);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        let output = 2;
        let result = number_of_subarrays(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let k = 2;
        let output = 16;
        let result = number_of_subarrays(nums, k);
        assert_eq!(result, output);
    }
}
