pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();
    for num in nums {
        *hm.entry(num).or_insert(0) += 1;
    }
    let mut res = vec![0; 2];
    let mut idx = 0;
    for (key, val) in hm.into_iter() {
        if val == 1 {
            res[idx] = key;
            idx += 1;
            if idx == 2 {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let output = vec![3, 5];
        let result = single_number(nums);
        assert_eq!(result, output);
    }
}
