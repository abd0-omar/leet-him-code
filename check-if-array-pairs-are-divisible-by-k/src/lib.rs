pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut remainder_count = std::collections::HashMap::new();

    for &num in &arr {
        let rem = (num % k + k) % k;
        *remainder_count.entry(rem).or_insert(0) += 1;
    }

    for num in arr {
        let rem = (num % k + k) % k;

        if rem == 0 {
            if remainder_count[&rem] % 2 != 0 {
                return false;
            }
        } else if remainder_count[&rem] != *remainder_count.get(&(k - rem)).unwrap_or(&0) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 7;
        let output = true;

        let result = can_arrange(arr, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        let output = false;
        let result = can_arrange(arr, k);
        assert_eq!(result, output);
    }
}
