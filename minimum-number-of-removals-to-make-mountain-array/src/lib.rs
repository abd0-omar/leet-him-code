pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    // the formula is as follows
    // n - inc - dec + 1 (+ 1, because we double counted the pivot)
    // 8 - 3 - 3 + 1
    // the length of the array - the elements that we will preserve
    // ignore the rest of the comments
    // //
    // // the optimal example
    // // two to remove on the left
    // // one to remove on the right
    // //
    // // inc
    // // 6, 5, 1 => (3)
    // // dec
    // // 6, 2, 1 => (3)
    // // we need to remove one pivot, because it is included in both lis & lds, remove one of them
    // //
    // // make "zero based" `1` to "one based" by adding 1, because there is `n` (one based number)
    // // let dec_result = n as i32 - i as i32 + 1 - dec;
    // //
    // // inc_result
    // // 4 - 3 + 1 -> 2 (remove 2 on the left)
    // // dec_result
    // // 8 - 4 + 1 - 3 + 1 -> 2 (remove 2 on the right)
    // //
    // // remove one pivot, because it is included in both lis & lds, remove one of them
    // //
    // //
    // // the formula is as follows
    // // n - inc - dec + 1 (+ 1, because we double counted the pivot)
    // // 8 - 3 - 3 + 1
    // //
    // //
    // // ignore comments below, I got confused down there
    // // [2, 1, 1, 5, 6, 2, 3, 1]
    // //
    // //
    // // pivot at 5 (idx 3)
    // // see how many to remove before the 5 and how many to remove after the 5
    // // before the 5, get the LIS, idx 3 - LIS is our answer
    // // after the 5, get the LDS, n - 3 - LDS is our answer
    let n = nums.len();
    let mut min = n as i32;

    let mut memory1 = vec![-1; n];
    let mut memory2 = vec![-1; n];

    for i in 1..n - 1 {
        // // 2
        let inc = lis(i, &nums, &mut memory1);

        // // 3
        let dec = lds(i, &nums, &mut memory2);
        // // 3 - 2 -> 1 + 1 -> 2

        if inc == 1 || dec == 1 {
            continue;
        }
        // // let inc_result = i as i32 - inc + 1;
        // // 8 - 3 - 3 -> 2 + 1 -> 3
        // // make "zero based" `1` to "one based" by adding 1, because there is `n` (one based number)
        // // let dec_result = n as i32 - i as i32 + 1 - dec;

        // // remove one pivot, because it is included in both lis & lds, remove one of them
        min = min.min(n as i32 - inc - dec + 1);
    }
    min
}

fn lis(st: usize, nums: &Vec<i32>, memory: &mut Vec<i32>) -> i32 {
    let mut ret = 1;
    if memory[st] != -1 {
        return memory[st];
    }
    for j in (0..st).rev() {
        if nums[j] < nums[st] {
            ret = ret.max(lis(j, nums, memory) + 1);
        }
    }
    memory[st] = ret;
    ret
}

fn lds(st: usize, nums: &Vec<i32>, memory: &mut Vec<i32>) -> i32 {
    let mut ret = 1;
    if memory[st] != -1 {
        return memory[st];
    }
    for j in st + 1..nums.len() {
        if nums[j] < nums[st] {
            ret = ret.max(lds(j, nums, memory) + 1);
        }
    }
    memory[st] = ret;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        let output = 3;
        let result = minimum_mountain_removals(nums);
        assert_eq!(result, output);
    }
}
