pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    // brute force
    // 23, 2, 4, 6, 7
    // i   j ->
    // for i in 0..nums.len() {
    //     let mut sum = nums[i];
    //     for j in i + 1..nums.len() {
    //         sum += nums[j];
    //         if sum % k == 0 {
    //             return true;
    //         }
    //     }
    // }
    // false

    // Math hit or miss
    let n = nums.len();
    if n < 2 {
        return false;
    }
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
    }
    // println!("prefix_sum={:?}", prefix_sum);
    // mod the prefix sum with k
    let mut mod_prefix = vec![0; n + 1];
    for i in 1..=n {
        mod_prefix[i] = prefix_sum[i] % k;
    }
    // println!("mod_prefix={:?}", mod_prefix);

    // we could calc prefix_sum & mod_prefix on the go in one loop but doesn't matter

    // we use hm to make sure they are of len 2 or more
    let mut hm = std::collections::HashMap::new();
    for (i, &m) in mod_prefix.iter().skip(1).enumerate() {
        // println!("m={:?}", m);
        // don't take first one
        if m == 0 && i != 0 {
            return true;
        }
        if let Some(idx) = hm.get(&m) {
            if i - idx >= 2 {
                return true;
            }
        }
        // put the oldest (farthest idx in map)
        hm.entry(m).or_insert(i);
    }
    false
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![23, 2, 4, 6, 7];
        let k = 6;
        let output = true;
        // Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
        let result = check_subarray_sum(nums, k);
        assert_eq!(result, output);
    }
}
