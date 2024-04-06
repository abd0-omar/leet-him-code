fn main() {
    println!("Hello, world!");

    let nums = vec![1, 3, 2, 3, 3];
    let k = 2;
    // Output: 6
    println!("{}", count_subarrays(nums, k));
}

/*
1,  3,  2,  3,  3
e|s

1,  3,  2,  3,  3
s   e

1,  3,  2,  3,  3
s           e

    1,  3,  2,  3,  3
0   0   1   1   2   3
*/
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let max = nums.iter().max().unwrap();
    let mut count = 0;

    let mut max_indices_vec = Vec::with_capacity(n);
    for (i, num) in nums.iter().enumerate() {
        if num == max {
            max_indices_vec.push(i as i64);
        }

        let freq_size = max_indices_vec.len();
        if freq_size >= k as usize {
            count += max_indices_vec[freq_size - k as usize] + 1;
        }
    }
    count
}
