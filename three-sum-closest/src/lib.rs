pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    // [-1, 2, 1, -4], t=1 -> 2
    // brute force
    // let mut min_diff = i32::MAX;
    // let mut closest_total = i32::MAX;

    // let n = nums.len();
    // for i in 0..n {
    //     for j in i + 1..n {
    //         for k in j + 1..n {
    //             let total = nums[i] + nums[j] + nums[k]; // -> 2
    //             let diff = (total - target).abs(); // -> 1
    //             dbg!(total);
    //             dbg!(diff);

    //             if diff < min_diff {
    //                 min_diff = diff;
    //                 closest_total = total;
    //             }
    //         }
    //     }
    // }

    // closest_total
    // works but time limit (expected)
    // now that we know how it could be solved, we can improve
    // sort
    // [-4, -1, 1, 2], t=1 -> 2
    // two pointers
    // loop from i and two pointer i+1 and n-1
    let mut nums = nums.clone();
    nums.sort_unstable();
    dbg!(&nums);
    let mut min_diff = i32::MAX;
    let mut closest_target = i32::MAX;
    let n = nums.len();
    for i in 0..n - 2 {
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            let total = nums[i] + nums[l] + nums[r];
            let diff = (total - target).abs();
            // dbg!(diff);
            // dbg!(total);
            if diff < min_diff {
                min_diff = diff;
                closest_target = total;
            }

            if total > target {
                r -= 1;
            } else if total < target {
                l += 1;
            } else {
                return total;
            }
        }
    }
    closest_target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let output = 2;
        let result = three_sum_closest(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![0, 0, 0];
        let target = 1;
        let output = 0;
        let result = three_sum_closest(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 1, 1, 0];
        let target = -100;
        let output = 2;
        let result = three_sum_closest(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![-100, -98, -2, -1];
        let target = -101;
        let output = -101;
        let result = three_sum_closest(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let nums = vec![4, 0, 5, -5, 3, 3, 0, -4, -5];
        let target = -2;
        let output = -2;
        let result = three_sum_closest(nums, target);
        assert_eq!(result, output);
    }
}
