pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    // from the first look, it looks obvious it's a straightforward prefix sum problem
    let n = n as usize;
    let new_n = n * (n + 1) / 2;
    dbg!(new_n);
    let mut prefix_sum = vec![0; new_n + n];
    // let mut i = 1;
    // while i < n + 1 {
    //     for j in i..new_n {
    //         let prev_nums = nums[(j - 1) % n];
    //         prefix_sum[j] = prefix_sum[j - 1] + prev_nums;
    //     }
    //     i += 2;
    //     dbg!(&prefix_sum); // [0, 1, 3, 6, 10]
    // }
    let mut i = 0;
    let mut n_down = n;
    let mut big_j = 0;
    loop {
        i += 1;
        let mut j = big_j;
        for _ in 0..n_down {
            prefix_sum[i] = prefix_sum[i - 1] + nums[j];
            i += 1;
            j += 1;
        }
        dbg!(&prefix_sum); // [0, 1, 3, 6, 10]
        n_down -= 1;
        big_j += 1;
        if i == new_n + n {
            break;
        }
    }

    let left_idx = (left - 1) as usize + n;
    let right_idx = (right - 1) as usize + n;
    dbg!(left_idx);
    dbg!(right_idx);
    prefix_sum.sort_unstable();
    // skip first n zeros
    // zero based index

    let sum: i64 = prefix_sum
        .into_iter()
        .skip(left_idx)
        .take(right_idx - left_idx + 1)
        .map(|x| x as i64)
        .sum::<i64>()
        % 1000_000_007;
    // dbg!(&prefix_sum);
    dbg!(sum);

    // left - right - 1
    // All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4.
    // After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10].
    // The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.
    // Example 2:

    sum as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 1;
        let right = 5;
        let output = 13;
        let result = range_sum(nums, n, left, right);
        assert_eq!(result, output);
    }
}
