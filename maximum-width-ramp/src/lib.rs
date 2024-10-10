pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    // it looks like the next bigger element problem but we need the last bigger element
    //                x
    // 6, 0, 8, 2, 1, 5
    //                x
    // 6, 0, 8, 2, 1, 5
    //
    //
    // next smaller
    // stack: 0, 5
    // the reverse of it
    // stack: 0, 6
    //
    // pops the elemnts if they're smaller than the cur
    // stack: 0, 5
    //
    // idk how monotonic stack would work
    //
    // brute force exists ik but ik it'll get
    // two pointers approach seems intuitive
    // but I don't know if it'll pass,
    // my two pointers idea is too slow compared to the editorial one which uses something like
    // prefix sum
    //
    // we can use binary search but the T and F are not consistent
    // we can use a hashmap to check for previous numbers that are lower than me, but that won't
    // work
    // we'll just try the two pointers method
    //
    // just looked at the stack solution from the editorial and it looks good,
    // this one is good too  https://leetcode.com/problems/maximum-width-ramp/solutions/666237/rust-solution-using-sort/?envType=daily-question&envId=2024-10-10
    //
    let mut stack = Vec::new();
    // monotonically decreasing stack
    for (idx, &num) in nums.iter().enumerate() {
        if let Some(&peek_idx) = stack.last() {
            if nums[peek_idx] > num {
                stack.push(idx);
            }
        } else {
            // first time assigning to the stack, "empty"
            stack.push(idx);
        }
    }

    dbg!(&stack);
    let mut max = 0;

    // go from the end and compare with the stack and get the max ramp
    for (idx, &num) in nums.iter().enumerate().rev() {
        while !stack.is_empty() && num >= nums[*stack.last().unwrap()] {
            let peek_idx = stack.pop().unwrap();
            max = max.max(idx - peek_idx);
        }
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![6, 0, 8, 2, 1, 5];
        let output = 4;
        let result = max_width_ramp(nums);
        assert_eq!(result, output);
    }
}
