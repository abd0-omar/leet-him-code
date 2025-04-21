// https://leetcode.com/problems/count-the-hidden-sequences/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        /*
        [1, -3, 4]
        choose arbitrary number
        it's not actually arbitrary we'll chosse `lower`
        but I wanted to use that word "arbitrary" اعتباطا

        then do the diff stuff
        [1, 2, -1, 3]
        bring minimum in range by lifting all the elements by lower - min here by 1 - -1 = 2
        [3, 4, 1, 5]
        then conceptually we want to lift all the elements by upper - max
        then our answer is upper -max + 1
        */
        let n = differences.len();
        let mut hidden = Vec::with_capacity(n + 1);
        hidden.push(lower);

        for i in 0..n {
            hidden.push(differences[i] + hidden[i]);
        }

        let mut min = hidden[0];
        let mut max = hidden[0];
        for &num in &hidden {
            min = min.min(num);
            max = max.max(num);
        }

        let shift = lower - min;

        for num in &mut hidden {
            *num += shift;
        }

        for &num in &hidden {
            if num < lower || num > upper {
                return 0;
            }
        }

        let remaining_shift = upper - max - shift;

        if remaining_shift >= 0 {
            remaining_shift + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let differences = vec![1, -3, 4];
        let lower = 1;
        let upper = 6;
        let output = 2;
        let result = Solution::number_of_arrays(differences, lower, upper);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let differences = vec![3, -4, 5, 1, -2];
        let lower = -4;
        let upper = 5;
        let output = 4;
        let result = Solution::number_of_arrays(differences, lower, upper);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let differences = vec![4, -7, 2];
        let lower = 3;
        let upper = 6;
        let output = 0;
        let result = Solution::number_of_arrays(differences, lower, upper);
        assert_eq!(result, output);
    }
}
