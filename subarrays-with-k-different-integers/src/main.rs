fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 1, 2, 3];
    let k = 2;
    println!("{}", subarrays_with_k_distinct(nums, k));
}

pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    _subarrays_with_k_distinct(&nums, k) - _subarrays_with_k_distinct(&nums, k - 1)
}

pub fn _subarrays_with_k_distinct(nums: &Vec<i32>, k: i32) -> i32 {
    let mut hm = std::collections::HashMap::new();
    let mut st = 0usize;
    let mut count = 0i32;
    for (end, num) in nums.iter().enumerate() {
        *hm.entry(num).or_insert(0) += 1;
        while hm.len() > k as usize {
            println!("hm={:?}", hm);
            println!("st={:?}", st);
            let st_freq = hm.get_mut(&nums[st]).unwrap();
            *st_freq -= 1;
            if *st_freq == 0 {
                hm.remove(&nums[st]);
            }
            st += 1;
        }
        count += (end - st + 1) as i32;
    }
    count
}
