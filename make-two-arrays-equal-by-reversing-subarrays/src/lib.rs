pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    //  [1, 2, 3, 4]
    //  [2, 4, 1, 3]
    // I guess if the freq equal then return true
    let mut freq = vec![0; 1001];
    for element in target {
        freq[element as usize] += 1;
    }

    // to check if they are equal, subtract freq from arr, so that final freq would be zeros
    for element in arr {
        let x = &mut freq[element as usize];
        *x -= 1;

        // early return
        if *x < 0 {
            return false;
        }
    }

    if freq.into_iter().any(|x| x != 0) {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let target = vec![1, 2, 3, 4];
        let arr = vec![2, 4, 1, 3];
        let output = true;
        let result = can_be_equal(target, arr);
        assert_eq!(result, output);
    }
}
