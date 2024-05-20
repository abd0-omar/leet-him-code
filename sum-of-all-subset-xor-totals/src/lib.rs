pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut result = Vec::new();
    _subset_xor_sum(&nums, 0, vec![], &mut result);
    println!("result={:?}", result);
    let mut final_result = 0;
    for subset in result {
        let mut cur_result = 0;
        for num in subset {
            cur_result ^= num;
        }
        println!("cur_result={:?}", cur_result);
        final_result += cur_result;
        println!("final_result={:?}", final_result);
    }
    final_result
}

pub fn _subset_xor_sum(
    nums: &Vec<i32>,
    start: usize,
    mut cur_vec: Vec<i32>,
    result_vec: &mut Vec<Vec<i32>>,
) {
    result_vec.push(cur_vec.clone()); // Add the current subset to the result

    for end in start..nums.len() {
        cur_vec.push(nums[end]);
        _subset_xor_sum(nums, end + 1, cur_vec.clone(), result_vec); // Recur with the next element
        cur_vec.pop();
    }
}

// other way
pub fn _subset_xor_sum2(
    nums: &Vec<i32>,
    idx: usize,
    mut cur_vec: Vec<i32>,
    result_vec: &mut Vec<Vec<i32>>,
) {
    if idx == nums.len() {
        result_vec.push(cur_vec);
        return;
    }

    cur_vec.push(nums[idx]);
    let _pick = _subset_xor_sum(nums, idx + 1, cur_vec.clone(), result_vec);
    cur_vec.pop();

    let _leave = _subset_xor_sum(nums, idx + 1, cur_vec, result_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![5, 1, 6];
        let output = 28;
        let result = subset_xor_sum(nums);
        assert_eq!(result, output);
        // Explanation: The 8 subsets of [5,1,6] are:
        // - The empty subset has an XOR total of 0.
        // - [5] has an XOR total of 5. X
        // - [1] has an XOR total of 1.
        // - [6] has an XOR total of 6.
        // - [5,1] has an XOR total of 5 XOR 1 = 4. X
        // - [5,6] has an XOR total of 5 XOR 6 = 3. X
        // - [1,6] has an XOR total of 1 XOR 6 = 7.
        // - [5,1,6] has an XOR total of 5 XOR 1 XOR 6 = 2. X
        // 0 + 5 + 1 + 6 + 4 + 3 + 7 + 2 = 28
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1];
        let output = 4;
        let result = subset_xor_sum(nums);
        assert_eq!(result, output);
    }
}
