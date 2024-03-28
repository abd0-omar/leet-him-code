fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
    let k = 2;
    // Output: 6
    println!("{}", max_subarray_length(nums, k));

    let nums = vec![1, 2, 3, 1, 1, 2, 3];
    let k = 2;
    // Output: 6
    println!("{}", max_subarray_length(nums, k));

    let nums = vec![1, 11];
    let k = 2;
    // Output: 6
    println!("{}", max_subarray_length(nums, k));
}

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut st = 0;
    let mut end = 0;
    let mut hm = std::collections::HashMap::new();
    let mut ans = 0i32;
    while end < nums.len() {
        let f = hm.entry(nums[end]).or_insert(0);
        *f += 1;
        if *f > k {
            while hm[&nums[end]] > k {
                *hm.entry(nums[st]).or_insert(0) -= 1;
                st += 1;
            }
        }
        //println!("end={:?}", end);
        //println!("st={:?}", st);
        ans = ans.max((end - st) as i32 + 1);
        end += 1;
    }
    ans
}
