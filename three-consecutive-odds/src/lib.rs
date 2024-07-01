pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut count = 0;
    for a in arr {
        if a % 2 == 1 {
            count += 1;
        } else {
            count = 0;
        }
        if count == 3 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        let output = true;
        // Explanation: [5,7,23] are three consecutive odds.
        let result = three_consecutive_odds(arr);
        assert_eq!(result, output);
    }
}
