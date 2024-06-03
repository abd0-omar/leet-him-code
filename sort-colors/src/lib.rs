pub fn sort_colors(nums: &mut Vec<i32>) {
    // selection sort
    // for i in 0..nums.len() - 1 {
    //     let mut cur_min_idx = i;
    //     for j in i + 1..nums.len() {
    //         if nums[cur_min_idx] > nums[j] {
    //             cur_min_idx = j;
    //         }
    //     }
    //     nums.swap(cur_min_idx, i);
    // }
    // println!("nums={:?}", nums);
    // count sort
    // let mut freq = vec![0; 3];
    // for &num in nums.iter() {
    //     freq[num as usize] += 1;
    // }
    // let mut idx = 0;
    // for i in 0..3 {
    //     for _ in 0..freq[i] {
    //         nums[idx] = i as i32;
    //         idx += 1;
    //     }
    // }
    // insertion sort
    for i in 1..nums.len() {
        let key = nums[i];
        let mut j = i as i32 - 1;
        while j >= 0 && nums[j as usize] > key {
            // shifting
            // 1 6 7 5
            // 1 6 7 7
            // 1 6 6 7
            // 1 5 6 7
            nums[j as usize + 1] = nums[j as usize];
            j -= 1;
        }
        nums[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 0, 2, 2, 1, 1, 0
        //
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, output);
    }
}
