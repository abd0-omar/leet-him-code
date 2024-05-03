pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let max_num = *nums.iter().max().unwrap();
    let min_num = *nums.iter().min().unwrap();
    let mut freq_array = vec![0; max_num.max(min_num.abs()) as usize + 1];

    for num in nums {
        if num.is_positive() {
            if freq_array[num as usize] == 0 || freq_array[num as usize] == -1 {
                freq_array[num as usize] += 2;
            }
        } else {
            println!("num={:?}", num);
            if freq_array[-num as usize] == 0 || freq_array[-num as usize] == 2 {
                freq_array[-num as usize] -= 1;
            }
        }
    }

    let mut max_result = -1;
    for (i, freq) in freq_array.iter().enumerate() {
        if freq == &1 {
            max_result = max_result.max(i as i32);
        }
    }

    max_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 10, 6, 7, -7, 1];
        let output = 7;
        let result = find_max_k(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![-10, 8, 6, 7, -2, -3];
        let output = -1;
        let result = find_max_k(nums);
        assert_eq!(result, output);
    }
}
