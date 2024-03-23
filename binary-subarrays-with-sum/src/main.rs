fn main() {
    println!("Hello, world!");
    let nums = vec![1, 0, 1, 0, 1];
    let goal = 2;
    // Output: 4
    println!("{}", num_subarrays_with_sum(nums, goal));
    // let nums = vec![0, 0, 0, 0, 0];
    // let goal = 0;
    // // Output: 4
    // println!("{}", num_subarrays_with_sum(nums, goal));
}
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    fn helper(x: i32, n: usize, nums: Vec<i32>) -> i32 {
        if x < 0 {
            return 0;
        }
        let mut res = 0;
        let (mut l, mut cur) = (0, 0);
        for r in 0..n {
            cur += nums[r];
            while cur > x {
                cur -= nums[l];
                l += 1;
            }
            res += r - l + 1;
        }
        res as _
    }
    helper(goal, nums.len(), nums.clone()) - helper(goal - 1, nums.len(), nums)
}
