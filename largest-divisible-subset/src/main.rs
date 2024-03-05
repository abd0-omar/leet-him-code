fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 4, 8];
    println!(
        "largest_divisible_subset={:?}",
        largest_divisible_subset(nums)
    );
    let nums = vec![1, 2, 3];
    println!(
        "largest_divisible_subset={:?}",
        largest_divisible_subset(nums)
    );
    let nums = vec![3, 4, 16, 8];
    println!(
        "largest_divisible_subset={:?}",
        largest_divisible_subset(nums)
    );
    // 4, 8, 16
}

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut memory = vec![vec![None; nums.len()]; nums.len()];
    _largest_divisible_subset(&nums, 0, None, &mut memory)
}

fn _largest_divisible_subset(
    nums: &[i32],
    idx: usize,
    prev: Option<usize>,
    memory: &mut Vec<Vec<Option<Vec<i32>>>>,
) -> Vec<i32> {
    if idx == nums.len() {
        return vec![];
    }

    if let Some(p) = prev {
        if let Some(ref ret) = memory[idx][p] {
            return ret.clone();
        }
    }

    let mut choice1 = vec![];
    if let Some(p) = prev {
        if nums[idx] % nums[p] == 0 {
            choice1 = _largest_divisible_subset(nums, idx + 1, Some(idx), memory);
            choice1.push(nums[idx]);
        }
    } else {
        choice1 = _largest_divisible_subset(nums, idx + 1, Some(idx), memory);
        choice1.push(nums[idx]);
    }

    let choice2 = _largest_divisible_subset(nums, idx + 1, prev, memory);

    let result = if choice1.len() > choice2.len() {
        choice1
    } else {
        choice2
    };

    if let Some(p) = prev {
        memory[idx][p] = Some(result.clone());
        return result;
    } else {
        return result;
    }
}
