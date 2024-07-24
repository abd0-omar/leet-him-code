pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let nums_string: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    dbg!(&nums_string);
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut nums_after_mapping = BinaryHeap::with_capacity(n);
    for (idx, string_num) in nums_string.into_iter().enumerate() {
        let mut result = String::with_capacity(string_num.len());
        for num in string_num.as_bytes() {
            let num = num - b'0';
            let num = mapping[num as usize];
            result.push(char::from_digit(num as u32, 10).unwrap());
        }
        // 007
        // let mut i = 0;
        // while result.as_bytes()[i] == b'0' && i < result.len() - 1 {
        //     i += 1;
        // }
        dbg!(&result);
        nums_after_mapping.push((Reverse(result.parse::<i32>().unwrap()), Reverse(idx)));
    }
    dbg!(&nums_after_mapping);
    println!("{:?}", nums_after_mapping);

    let mut result = Vec::with_capacity(n);
    while let Some(pop) = nums_after_mapping.pop() {
        result.push(nums[pop.1 .0]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
        let nums = vec![991, 338, 38];
        let output = vec![338, 38, 991];
        let result = sort_jumbled(mapping, nums);
        assert_eq!(result, output);
    }
}
