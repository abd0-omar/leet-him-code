fn main() {
    println!("Hello, world!");
    // Example 1:

    // let nums = vec![1, 1, 1];
    // let k = 2;
    // // Output: 2
    // // Example 2:
    // println!("{:?}", subarray_sum(nums, k));

    // let nums = vec![1, 2, 3];
    // let k = 3;
    // // Output: 2
    // println!("{:?}", subarray_sum(nums, k));

    let nums = vec![-1, -1, 1];
    let k = 0;
    // Output: 2
    println!("{:?}", subarray_sum(nums, k));
}

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    if n == 1 {
        if k == nums[0] {
            return 1;
        } else {
            return 0;
        }
    }
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
    }

    // let mut st = 0;
    // let mut end = 1;
    // // if end - st < 2, end ++
    // // if end - st >= 2, st ++
    // let mut count = 0;
    // while end <= n && st < end {
    //     let subarray_sum = prefix_sum[end] - prefix_sum[st];
    //     println!("subarray_sum{}", subarray_sum);
    //     if subarray_sum < k {
    //         end += 1;
    //     } else {
    //         if subarray_sum == k {
    //             count += 1;
    //         }
    //         st += 1;
    //     }
    // }
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..=n {
            let mut diff = prefix_sum[j] - prefix_sum[i];
            if diff == k {
                count += 1;
            }
        }
    }

    count
}
