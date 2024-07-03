pub fn min_difference(nums: Vec<i32>) -> i32 {
    // choose lowest 3 and change them or
    // choose biggest 3 and change them
    // make a 3 len array of lowest and biggest
    // biggest
    // [5, 3, 2, 4];
    // [5, _, _]  0
    // [5, 3, _]      // [3, 5, _] 1
    // [3, 5, 2]      // [2, 3, 5] 2
    // if nums[4] > biggest[0], then change
    // [4, 3, 5]      // [3, 4, 5]
    // smallest is the inverse way of biggest
    //
    // found out it's actually the 4th
    if nums.len() <= 4 {
        return 0;
    }
    let mut biggest = vec![(None, 0); 4];
    let mut smallest = vec![(None, 0); 4];
    for (i, &num) in nums.iter().enumerate() {
        if i >= 4 {
            biggest.sort_unstable();
            smallest.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            if num > biggest[0].0.unwrap() {
                biggest[0] = (Some(num), i);
            }
            if num < smallest[0].0.unwrap() {
                smallest[0] = (Some(num), i);
            }
        } else {
            biggest[i] = (Some(num), i);
            smallest[i] = (Some(num), i);
        }
    }
    biggest.sort_unstable();
    smallest.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    // 4th biggest
    dbg!(&biggest);
    // 4th smallest
    dbg!(&smallest);

    let mut min_diff = i32::MAX;
    for i in 0..4 {
        min_diff = min_diff.min(biggest[4 - i - 1].0.unwrap() - smallest[i].0.unwrap());
    }

    min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![5, 3, 2, 4];
        let output = 0;
        // Explanation: We can make at most 3 moves.
        // In the first move, change 2 to 3. nums becomes [5,3,3,4].
        // In the second move, change 4 to 3. nums becomes [5,3,3,3].
        // In the third move, change 5 to 3. nums becomes [3,3,3,3].
        // After performing 3 moves, the difference between the minimum and maximum is 3 - 3 = 0.
        let result = min_difference(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 5, 0, 10, 14];
        let output = 1;
        // Explanation: We can make at most 3 moves.
        // In the first move, change 5 to 0. nums becomes [1,0,0,10,14].
        // In the second move, change 10 to 0. nums becomes [1,0,0,0,14].
        // In the third move, change 14 to 1. nums becomes [1,0,0,0,1].
        // After performing 3 moves, the difference between the minimum and maximum is 1 - 0 = 1.
        // It can be shown that there is no way to make the difference 0 in 3 moves.
        let result = min_difference(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 100, 20];
        let output = 0;
        // Explanation: We can make at most 3 moves.
        // In the first move, change 100 to 7. nums becomes [3,7,20].
        // In the second move, change 20 to 7. nums becomes [3,7,7].
        // In the third move, change 3 to 7. nums becomes [7,7,7].
        // After performing 3 moves, the difference between the minimum and maximum is 7 - 7 = 0.
        let result = min_difference(nums);
        assert_eq!(result, output);
    }
}
