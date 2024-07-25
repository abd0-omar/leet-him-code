pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    quick_sort(&mut nums, 0, n - 1);
    nums
}

fn quick_sort(nums: &mut Vec<i32>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let partition_idx = partition(nums, low, high);

    if partition_idx > 0 {
        quick_sort(nums, low, partition_idx - 1);
    }
    quick_sort(nums, partition_idx + 1, high);
}

fn partition(nums: &mut Vec<i32>, low: usize, high: usize) -> usize {
    use rand::Rng;

    let random_pivot_index = rand::thread_rng().gen_range(low..=high);
    nums.swap(random_pivot_index, high);

    let pivot = nums[high];

    let mut i = low as i32 - 1;

    for j in low..high {
        if nums[j] < pivot {
            i += 1;
            nums.swap(j, i as usize);
        }
    }

    nums.swap((i + 1) as usize, high);
    (i + 1) as usize
    // 1 7 3 5 | 4
    //ij        pivot
    // 1 7 3 5 | 4
    // swap in place
    // i j
    // 1 7 3 5 | 4
    // i   j
    // 1 7 3 5 | 4
    // i go forward one step and swap
    //   i j
    // 1 3 7 5 | 4
    //   i j
    // 1 3 7 5 | 4
    //   i   j
    // 1 3 7 5 | 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let output = vec![0, 0, 1, 1, 2, 5];
        let result = sort_array(nums);
        assert_eq!(result, output);
    }
}
