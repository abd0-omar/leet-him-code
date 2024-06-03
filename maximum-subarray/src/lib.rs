pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut i = 1;
    let mut max = nums[0];
    let mut total_so_far = nums[0];
    while i < nums.len() {
        //dbg!(total_so_far + nums[i]);
        if nums[i] > total_so_far + nums[i] {
            // skip total_so_far
            total_so_far = nums[i];
        } else {
            total_so_far += nums[i];
        }
        //println!("total_so_far={:?}", total_so_far);
        max = max.max(total_so_far);
        i += 1
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        // -2, 1
        // 1 > -2
        // remove -2 and start from 1
        // 1, -3
        // -3 < 1
        // continue with 1 but ur max is 1 for now and add -3 to 1 -> -2
        // -2, 4
        // remove -2 and start from 4
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let output = 6;
        let result = max_sub_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2];
        let output = 3;
        let result = max_sub_array(nums);
        assert_eq!(result, output);
    }
}
