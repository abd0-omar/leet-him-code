// https://leetcode.com/problems/count-good-triplets/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        // brute-force
        let n = arr.len();
        let mut result = 0;
        for i in 0..n - 2 {
            let x = arr[i];
            for j in i + 1..n - 1 {
                let y = arr[j];
                for k in j + 1..n {
                    let z = arr[k];
                    if (x - y).abs() <= a && (y - z).abs() <= b && (x - z).abs() <= c {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![3, 0, 1, 1, 9, 7];
        let a = 7;
        let b = 2;
        let c = 3;
        let output = 4;
        let result = Solution::count_good_triplets(arr, a, b, c);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 1, 2, 2, 3];
        let a = 0;
        let b = 0;
        let c = 1;
        let output = 0;
        let result = Solution::count_good_triplets(arr, a, b, c);
        assert_eq!(result, output);
    }
}
