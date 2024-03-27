fn main() {
    println!("Hello, world!");
    // let nums = vec![10, 5, 2, 6];
    // let k = 100;
    // 8
    // println!("{}", num_subarray_product_less_than_k(nums, k));
    let nums = vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3];
    let k = 19;
    // 18
    println!("{}", num_subarray_product_less_than_k(nums, k));
}
// pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
//     let n = nums.len();
//     let mut prefix_mult: Vec<i64> = vec![1; n + 1];
//     for i in 1..=n {
//         prefix_mult[i] = prefix_mult[i - 1] * nums[i - 1] as i64;
//     }
//     println!("prefix_mult={:?}", prefix_mult);
//
//     let mut count = 0;
//     for i in 0..n {
//         let first = prefix_mult[i];
//         for j in (i + 1..=n).rev() {
//             // for j in i + 1..=n {
//             let second = prefix_mult[j];
//             if first == 0 {
//                 continue;
//             }
//             let mult = second / first;
//             if mult < k as i64 {
//                 count += j - i;
//                 break;
//                 // count += 1;
//             }
//             // } else {
//             // break;
//             // }
//         }
//     }
//
//     count as _
// }

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }
    let mut answer = 0;
    let mut p = 1;
    let mut j = 0;
    for (i, num) in nums.iter().enumerate() {
        p *= num;
        while p >= k {
            p /= nums[j];
            j += 1;
        }
        answer += i + 1 - j;
    }
    answer as i32
}
