pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    // I guess get the max value, then the max subarray with the max value
    // let's try brute-force first
    let mut k = nums.iter().max().cloned().unwrap();
    let n = nums.len();

    // oh just deleted this part and now the tle is gone, nice
    // for i in 0..n {
    //     for j in i + 1..n {
    //         let cur_k = nums[i] & nums[j];
    //         k = k.max(cur_k);
    //     }
    // }

    dbg!(k);

    let mut max_count = 0;
    let mut cur_count = 0;
    for i in 0..n {
        if nums[i] == k {
            cur_count += 1;
        } else {
            cur_count = 0;
        }
        dbg!(cur_count);
        max_count = max_count.max(cur_count);
    }

    max_count
    // gets tle as expected, but works, so our only way is up now, and we are already up 💯
}

pub fn cleaner_longest_subarray(nums: Vec<i32>) -> i32 {
    let mut k = nums.iter().max().cloned().unwrap();

    let (mut max_count, mut cur_count) = (0, 0);
    for num in nums {
        if num == k {
            cur_count += 1;
            max_count = max_count.max(cur_count);
        } else {
            cur_count = 0;
        }
    }

    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        let output = 2;
        // Explanation:
        // The maximum possible bitwise AND of a subarray is 3.
        // The longest subarray with that value is [3,3], so we return 2.
        let result = longest_subarray(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![
            96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979,
        ];
        let output = 1;
        let result = longest_subarray(nums);
        assert_eq!(result, output);
    }
}
