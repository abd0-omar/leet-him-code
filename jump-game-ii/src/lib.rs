pub fn jump(nums: Vec<i32>) -> i32 {
    let mut memo = vec![-1; nums.len()];
    _jump(&nums, 0, &mut memo)
}

pub fn _jump(nums: &Vec<i32>, idx: usize, memo: &mut Vec<i32>) -> i32 {
    if idx >= nums.len() - 1 {
        return 0;
    }

    if memo[idx] != -1 {
        return memo[idx];
    }

    let mut ans = nums.len();
    for i in 1..=nums[idx] {
        let new_idx = idx + i as usize;
        if new_idx < nums.len() {
            ans = ans.min(_jump(nums, new_idx, memo) as usize + 1);
        }
    }
    memo[idx] = ans as i32;
    ans as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(jump(nums), 2);
    }
}
