pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let mut queue = std::collections::VecDeque::new();
    let mut result = 0;
    for i in 0..nums.len() {
        while !queue.is_empty() && i > queue.front().unwrap() + k as usize - 1 {
            queue.pop_front();
        }
        // how many flips we done
        if (nums[i] + queue.len() as i32) % 2 == 0 {
            // we have to flip but window not enough
            if i + k as usize > nums.len() {
                return -1;
            }
            queue.push_back(i);
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
        let k = 3;
        let output = 3;
        // Explanation:
        // Flip nums[0],nums[1],nums[2]: nums becomes [1,1,1,1,0,1,1,0]
        // Flip nums[4],nums[5],nums[6]: nums becomes [1,1,1,1,1,0,0,0]
        // Flip nums[5],nums[6],nums[7]: nums becomes [1,1,1,1,1,1,1,1]
        let result = min_k_bit_flips(nums, k);
        assert_eq!(result, output);
    }
}
