pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result_subset = Vec::new();
    let mut nums = nums.clone();
    nums.sort_unstable();
    _subsets_with_dup(&nums, 0, vec![], &mut result_subset);
    result_subset
}

//                             idx = 0 [1, 2, 2]
//                     idx = 1 (1)[2, 2]                         [1, 2, 2]
//              idx = 2 (1, 2)[2]               (1)[2, 2x]
//          idx = 3 (1, 2, 2)[]  (1, 2)[]       (1, 2x)[]       (1)[]

pub fn _subsets_with_dup(
    nums: &Vec<i32>,
    st: usize,
    mut cur_subset: Vec<i32>,
    result_subset: &mut Vec<Vec<i32>>,
) -> () {
    // cur_subset.sort_unstable();
    if !result_subset.contains(&cur_subset) {
        result_subset.push(cur_subset.clone());
    }

    for end in st..nums.len() {
        if end != st && nums[end] == nums[end - 1] {
            continue;
        }
        cur_subset.push(nums[end]);
        _subsets_with_dup(nums, end + 1, cur_subset.clone(), result_subset);
        cur_subset.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 2];
        let output = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        let result = subsets_with_dup(nums);
        assert_eq!(result, output);
    }
}
