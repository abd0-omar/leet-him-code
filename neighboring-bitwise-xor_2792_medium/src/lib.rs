// https://leetcode.com/problems/neighboring-bitwise-xor/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        // xor-sum of the derived array should be zero
        // that's the 2nd hint and the solution
        // but we could do it in a "brute-force" way
        // derived
        // [1, 1, 0]
        // original
        // [0] // default value
        //
        // to make a "1", the xor of the first adj. values should be "1", (different bits)
        //
        // derived
        // [1, 1, 0]
        // original
        // [0] // default value
        //
        // derived
        // [1, 1, 0]
        // original
        // [0, 1]
        //
        // derived
        // [1, 1, 0]
        // original
        // [0, 1, 1]
        // true
        //
        // false example
        // [1, 0]
        // [0] //default val
        //
        // [1, 0]
        // [0, 1]
        // false
        let n = derived.len();
        let mut original = vec![0; n];
        for i in 0..n {
            if derived[i] == 1 {
                if i == n - 1 {
                    if original[i] == 0 {
                        if original[0] != 1 {
                            return false;
                        } else {
                            return true;
                        }
                    } else if original[i] == 1 {
                        if original[0] != 0 {
                            return false;
                        } else {
                            return true;
                        }
                    }
                } else {
                    if original[i] == 0 {
                        original[i + 1] = 1;
                    } else if original[i] == 1 {
                        original[i + 1] = 0;
                    }
                }
            } else if derived[i] == 0 {
                if i == n - 1 {
                    {
                        if original[i] == 0 {
                            if original[0] != 0 {
                                return false;
                            } else {
                                return true;
                            }
                        } else if original[i] == 1 {
                            if original[0] != 1 {
                                return false;
                            } else {
                                return true;
                            }
                        }
                    }
                } else {
                    if original[i] == 1 {
                        original[i + 1] = 1;
                    }
                }
            }
        }
        // dbg!(&original);
        // better solution to check just the first and last bits after operations, so it would be
        // O(1) space
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let derived = vec![1, 1, 0];
        let output = true;
        let result = Solution::does_valid_array_exist(derived);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let derived = vec![1, 1];
        let output = true;
        let result = Solution::does_valid_array_exist(derived);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let derived = vec![1, 0];
        let output = false;
        let result = Solution::does_valid_array_exist(derived);
        assert_eq!(result, output);
    }
}
