pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    // first thought
    // prefix sum with a hashmap
    //         [4, 5, 0, -2, -3, 1]
    //      [0, 4, 9, 9,  7,  4, 5]
    //       (4 - 9).abs() = 5
    //       what if the no. sum is more than 5 but divisble by 5
    //       or less than like this [5, 0, -2, -3] -> 0
    //       so what our hashmap have?
    //
    //       we could mod it
    //       [4, 4, 4, 2, 4, 0]
    //        i  j -> 3
    //           i  j -> 2
    //              i  j  -> 1
    //                       i -> 1
    //        total 7
    //        a freq hashmap
    let mut hm = std::collections::HashMap::new();
    let mut result = 0;
    let mut prefix_sum = 0;
    hm.insert(0, 1);
    for num in nums {
        prefix_sum += num;
        // avoid negative remainder
        let mod_prefix_sum = ((prefix_sum % k) + k) % k;
        // if num % k == 0 {
        //     result += 1;
        //     continue;
        // }
        // if mod_prefix_sum == 0 {
        //     result += 1;
        // }

        if let Some(found_freq) = hm.get(&mod_prefix_sum) {
            println!("found_freq={:?}", found_freq);
            result += found_freq;
        }
        hm.entry(mod_prefix_sum)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        let output = 7;
        // Explanation: There are 7 subarrays with a sum divisible by k = 5:
        // [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
        let result = subarrays_div_by_k(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![-1, 2, 9];
        //             [-1, 1, 10]
        //             [-1, 1, 0]
        let k = 2;
        let output = 2;
        let result = subarrays_div_by_k(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![2, -2, 2, -4];
        //             [2,  0, 2, -2]
        //             [4,  6, 4,  4]
        let k = 6;
        let output = 2;
        let result = subarrays_div_by_k(nums, k);
        assert_eq!(result, output);
    }
}
