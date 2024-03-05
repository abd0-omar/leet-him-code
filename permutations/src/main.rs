fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3];
    println!("{:?}", permute(nums));
    let nums = vec![0, 1];
    println!("{:?}", permute(nums));
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    _permute(&nums, 0, vec![])
}

pub fn _permute(nums: &Vec<i32>, idx: usize, mut v: Vec<i32>) -> Vec<Vec<i32>> {
    if idx == nums.len() {
        return vec![v];
    }

    let mut result = vec![];
    for i in 0..nums.len() {
        if !v.contains(&nums[i]) {
            v.push(nums[i]);
            result.extend(_permute(nums, idx + 1, v.clone()));
            v.pop();
        }
    }
    result
}
