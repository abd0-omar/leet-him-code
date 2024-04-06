fn main() {
    println!("Hello, world!");
    let mut nums = vec![1, 2, 3];
    next_permutation(&mut nums);
}

// or example, for arr = [1,2,3],
// the following are all the permutations of arr:
// [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut rezult = Vec::new();
    _next_permutation(nums, 0, &mut rezult, Vec::new());
    rezult.sort_unstable();
    println!("rezult={:?}", rezult);

    todo!()
}

pub fn _next_permutation(
    nums: &Vec<i32>,
    idx: usize,
    rezult: &mut Vec<Vec<i32>>,
    cur_num: Vec<i32>,
) {
    if idx == nums.len() {
        rezult.push(cur_num);
        return;
    }

    for num in nums.iter() {
        if !cur_num.contains(num) {
            let mut new_cur_num = cur_num.clone();
            new_cur_num.push(*num);
            _next_permutation(nums, idx + 1, rezult, new_cur_num);
        }
    }
}
