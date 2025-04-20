// https://leetcode.com/problems/rabbits-in-forest/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        // [2] => 3
        // [2, 2] => 3
        // [2, 2, 2] => 3
        // [2, 2, 2, 2] => 6
        // count me and 2 other
        let mut hm = std::collections::HashMap::new();
        let mut result = 0;
        use std::collections::hash_map::Entry;
        for answer in answers {
            // edge case
            if answer == 0 {
                result += 1;
            } else {
                match hm.entry(answer) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(0);
                        result += answer + 1;
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        let count = occupied_entry.get_mut();
                        if *count == answer {
                            *count = 0;
                            result += answer + 1
                        } else {
                            *count += 1;
                        }
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
        let answers = vec![1, 1, 2];
        let output = 5;
        let result = Solution::num_rabbits(answers);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let answers = vec![10, 10, 10];
        let output = 11;
        let result = Solution::num_rabbits(answers);
        assert_eq!(result, output);
    }
}
