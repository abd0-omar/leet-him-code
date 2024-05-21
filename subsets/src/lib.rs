pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result_vec = Vec::new();
    _subsets_slices(&nums, vec![], &mut result_vec);
    result_vec
}

pub fn _subsets_slices(nums: &[i32], mut cur_vec: Vec<i32>, result_vec: &mut Vec<Vec<i32>>) -> () {
    result_vec.push(cur_vec.clone());

    for (i, &num) in nums.iter().enumerate() {
        cur_vec.push(num);
        _subsets_slices(&nums[i + 1..], cur_vec.clone(), result_vec);
        cur_vec.pop();
    }
}

pub fn _subsets_pick_or_leave(
    nums: &Vec<i32>,
    idx: usize,
    mut cur_vec: Vec<i32>,
    result_vec: &mut Vec<Vec<i32>>,
) -> () {
    if idx == nums.len() {
        result_vec.push(cur_vec);
        return;
    }

    cur_vec.push(nums[idx]);
    let _pick = _subsets_pick_or_leave(nums, idx + 1, cur_vec.clone(), result_vec);
    cur_vec.pop();

    let _leave = _subsets_pick_or_leave(nums, idx + 1, cur_vec, result_vec);
}

pub fn _subsets_range(
    nums: &Vec<i32>,
    st: usize,
    mut cur_vec: Vec<i32>,
    result_vec: &mut Vec<Vec<i32>>,
) -> () {
    result_vec.push(cur_vec.clone());

    for end in st..nums.len() {
        cur_vec.push(nums[end]);
        _subsets_range(nums, end + 1, cur_vec.clone(), result_vec);
        cur_vec.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];
        let mut output = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        output.sort();
        let mut result = subsets(nums);
        result.sort();
        assert_eq!(result, output);
    }
}
