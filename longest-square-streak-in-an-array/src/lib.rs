use std::collections::HashSet;

pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
    // hashset and if in the test case example
    // if 2, 4, 16 worked
    // then don't try at 4 or 16 cuz it'll give a smaller answer
    let lookup = HashSet::from_iter(nums.iter().cloned());
    let mut visited = HashSet::new();

    let mut max_count = 0;
    for &num in nums.iter() {
        if !visited.contains(&num) {
            let mut count = 0;
            check(num, &mut visited, &lookup, &nums, &mut count);
            max_count = max_count.max(count);
        }
    }
    if max_count == 0 {
        -1
    } else {
        max_count
    }
}

fn check(
    num: i32,
    visited: &mut HashSet<i32>,
    lookup: &HashSet<i32>,
    nums: &Vec<i32>,
    count: &mut i32,
) {
    visited.insert(num);
    *count += 1;
    num.checked_mul(num).map(|next_num| {
        if lookup.contains(&next_num) {
            check(next_num, visited, lookup, nums, count);
        } else {
            return;
        }
    });

    // if let Some(n_num) = next_num {
    //     if lookup.contains(&n_num) {
    //         check(n_num, visited, lookup, nums, count);
    //     } else {
    //         return;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![4, 3, 6, 16, 8, 2];
        let output = 3;
        let result = longest_square_streak(nums);
        assert_eq!(result, output);
    }
}
