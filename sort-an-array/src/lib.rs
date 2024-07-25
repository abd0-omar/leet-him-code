pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n <= 1 {
        return nums;
    }
    merge_sort(&nums)
}

// merge sort
pub fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    let mut tmp = vec![-1; vec.len()];
    let mut inversions_count = 0;
    let mut vec_clone = vec.clone();
    _merge_sort(
        &mut vec_clone,
        0,
        vec.len() - 1,
        &mut tmp,
        &mut inversions_count,
    );
    tmp
}

fn _merge_sort(
    vec: &mut Vec<i32>,
    st: usize,
    end: usize,
    tmp: &mut Vec<i32>,
    inversions_count: &mut i32,
) {
    if st >= end {
        return;
    }

    let mid = st + (end - st) / 2;

    _merge_sort(vec, st, mid, tmp, inversions_count);
    _merge_sort(vec, mid + 1, end, tmp, inversions_count);

    _join_sorted_arrays(vec, st, mid, end, tmp, inversions_count);
}

fn _join_sorted_arrays(
    vec: &mut Vec<i32>,
    st: usize,
    mid: usize,
    end: usize,
    tmp: &mut Vec<i32>,
    inversions_count: &mut i32,
) {
    let mut i = st;
    let mut j = mid + 1;
    let mut k = st;

    while i <= mid && j <= end {
        if vec[i] <= vec[j] {
            tmp[k] = vec[i];
            i += 1;
        } else {
            tmp[k] = vec[j];
            j += 1;
            *inversions_count += (mid + 1 - i) as i32;
        }
        k += 1;
    }

    while i <= mid {
        tmp[k] = vec[i];
        i += 1;
        k += 1;
    }

    while j <= end {
        tmp[k] = vec[j];
        j += 1;
        k += 1;
    }

    for index in st..=end {
        vec[index] = tmp[index];
    }
}

// basic random pivot quick sort
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
