// better approach would be having a hashmap of the numbers and checking if the nuumber -
// difference is in the hashmap or not and do the pick or leave pattern with hashamp[num] += 1 and
// hashmap[num] -= 1 as do and undo in backtracking
pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    _subsets_with_count_range(&nums, k, vec![], &mut count, 0);
    count
}

pub fn _subsets_with_count_range(
    nums: &Vec<i32>,
    k: i32,
    mut cur_subset: Vec<i32>,
    count: &mut i32,
    st: usize,
) -> () {
    if !cur_subset.is_empty() {
        // cur_subset.sort_unstable();
        let mut is_beautiful = true;
        'outer: for i in 0..cur_subset.len() {
            for j in i + 1..cur_subset.len() {
                if (cur_subset[j] - cur_subset[i]).abs() == k {
                    is_beautiful = false;
                    break 'outer;
                }
            }
        }
        if is_beautiful {
            *count += 1;
        }
    }

    for end in st..nums.len() {
        cur_subset.push(nums[end]);
        _subsets_with_count_range(nums, k, cur_subset.clone(), count, end + 1);
        cur_subset.pop();
    }
}

pub fn _subsets_with_count(
    nums: &Vec<i32>,
    k: i32,
    idx: usize,
    mut cur_subset: Vec<i32>,
    count: &mut i32,
) -> () {
    if idx == nums.len() {
        if !cur_subset.is_empty() {
            // cur_subset.sort_unstable();
            let mut is_beautiful = true;
            'outer: for i in 0..cur_subset.len() {
                for j in i + 1..cur_subset.len() {
                    if (cur_subset[j] - cur_subset[i]).abs() == k {
                        is_beautiful = false;
                        break 'outer;
                    }
                }
            }
            if is_beautiful {
                *count += 1;
            }
        }
        return;
    }

    let _leave = _subsets_with_count(nums, k, idx + 1, cur_subset.clone(), count);

    cur_subset.push(nums[idx]);
    let _pick = _subsets_with_count(nums, k, idx + 1, cur_subset.clone(), count);
    cur_subset.pop();
}

pub fn _subsets(
    nums: &Vec<i32>,
    k: i32,
    idx: usize,
    mut cur_subset: Vec<i32>,
    total_subset: &mut Vec<Vec<i32>>,
) -> () {
    if idx == nums.len() {
        if !cur_subset.is_empty() {
            total_subset.push(cur_subset);
        }
        return;
    }

    let _leave = _subsets(nums, k, idx + 1, cur_subset.clone(), total_subset);

    cur_subset.push(nums[idx]);
    let _pick = _subsets(nums, k, idx + 1, cur_subset.clone(), total_subset);
    cur_subset.pop();
}

pub fn _beautiful_subsets(
    nums: &Vec<i32>,
    k: i32,
    idx: usize,
    prev: Option<usize>,
    memory: &mut Vec<Vec<Option<i32>>>,
) -> i32 {
    if idx == nums.len() {
        return 0;
    }

    if let Some(p) = prev {
        if let Some(ret) = memory[idx][p] {
            return ret;
        }
    }

    let leave = _beautiful_subsets(nums, k, idx + 1, prev, memory);

    let pick = match prev {
        Some(p) => {
            if (nums[idx] - nums[p]).abs() != k {
                _beautiful_subsets(nums, k, idx + 1, Some(idx), memory) + 1
            } else {
                0
            }
        }
        None => _beautiful_subsets(nums, k, idx + 1, Some(idx), memory) + 1,
    };

    let result = pick + leave;
    if let Some(p) = prev {
        memory[idx][p] = Some(result);
    }
    result
}

pub fn _beautiful_subsets_in_vec(
    nums: &Vec<i32>,
    k: i32,
    idx: usize,
    prev: Option<usize>,
    mut cur_subset: Vec<i32>,
    total_subset: &mut Vec<Vec<i32>>,
) -> () {
    if idx == nums.len() {
        total_subset.push(cur_subset);
        return;
    }

    let _leave =
        _beautiful_subsets_in_vec(nums, k, idx + 1, prev, cur_subset.clone(), total_subset);

    match prev {
        Some(p) => {
            if (nums[idx] - nums[p]).abs() != k {
                cur_subset.push(nums[idx]);
                let _pick = _beautiful_subsets_in_vec(
                    nums,
                    k,
                    idx + 1,
                    Some(idx),
                    cur_subset.clone(),
                    total_subset,
                );
                cur_subset.pop();
            }
        }
        None => {
            cur_subset.push(nums[idx]);
            let _pick = _beautiful_subsets_in_vec(
                nums,
                k,
                idx + 1,
                Some(idx),
                cur_subset.clone(),
                total_subset,
            );
            cur_subset.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 4, 6];
        let k = 2;
        let output = 4;
        let result = beautiful_subsets(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1];
        let k = 1;
        let output = 1;
        let result = beautiful_subsets(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![4, 2, 5, 9, 10, 3];
        let k = 1;
        let output = 23;
        let result = beautiful_subsets(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![10, 4, 5, 7, 2, 1];
        let k = 3;
        let output = 23;
        let result = beautiful_subsets(nums, k);
        assert_eq!(result, output);
    }
}
