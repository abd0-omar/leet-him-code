pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let mut result = Vec::new();
    subsets(&nums, Vec::new(), &mut result, 0);
    dbg!(&result);
    let max_or = nums.into_iter().fold(0, |acc, x| acc | x);
    dbg!(max_or);
    let mut count = 0;
    for rezz in result {
        let cur_or = rezz.iter().fold(0, |acc, x| acc | x);
        if cur_or == max_or {
            count += 1;
        }
    }
    count
    // input so small that doing recursion and nested loops is fine
    // further improvement could be added
    // generate subsets and count each subset equal to max_or and utilize memoization
}

// descriptive comment
// subsets
fn subsets(nums: &Vec<i32>, mut cur_result: Vec<i32>, result: &mut Vec<Vec<i32>>, idx: usize) {
    if idx == nums.len() {
        result.push(cur_result.clone());
        return;
    }
    dbg!(&cur_result);
    // take
    cur_result.push(nums[idx]);
    let _take = subsets(nums, cur_result.clone(), result, idx + 1);
    cur_result.pop();
    // leave
    let _leave = subsets(nums, cur_result, result, idx + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![3, 2, 1, 5];
        let output = 6;
        let result = count_max_or_subsets(nums);
        assert_eq!(result, output);
    }
}
