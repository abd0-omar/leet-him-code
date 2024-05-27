pub fn special_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    for x in 0..1000 {
        let bigger_than_x = nums.iter().filter(|f| **f >= x as i32).count();
        if bigger_than_x == x {
            return x as _;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 5];
        let output = 2;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![0, 0];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        // 0, 0, 3, 4, 4
        let nums = vec![0, 4, 3, 0, 4];
        let output = 3;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![3, 6, 7, 7, 0];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let nums = vec![1, 1, 2];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }
}
