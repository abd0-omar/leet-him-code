pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    // get the max and min of the first 0 index elements
    let (mut min, mut max) = (nums[0][0], nums[0][0]);
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut min_heap = BinaryHeap::new();
    for (idx, num) in nums.iter().enumerate() {
        min = min.min(num[0]);
        max = max.max(num[0]);
        // min_val, idx of list, idx of element in that list
        min_heap.push((Reverse(num[0]), idx, 0))
    }
    let mut res = vec![min, max];

    loop {
        dbg!(&min_heap);
        let (Reverse(_cur_min_number), list_idx, mut idx_in_list) = min_heap.pop().unwrap();
        idx_in_list += 1;
        if idx_in_list == nums[list_idx].len() {
            break;
        }

        let next_element = nums[list_idx][idx_in_list];
        min_heap.push((Reverse(next_element), list_idx, idx_in_list));

        max = max.max(next_element);
        min = min_heap.peek().unwrap().0 .0;

        if max - min < res[1] - res[0] {
            res[1] = max;
            res[0] = min;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        let output = vec![20, 24];
        let result = smallest_range(nums);
        assert_eq!(result, output);
    }
}
