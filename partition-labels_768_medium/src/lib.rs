// https://leetcode.com/problems/partition-labels/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // my main idea was to track the last element but my own version would
        // be maintaining a hashset that has the unique letters and an int
        // unique_count_completed, if the unique_count_completed == hashet.len()
        // then I've reached the end of the partition and reset the count and
        // the hashset.
        // cool solution and all but to maintain the last element as above and
        // added  to it, maintain the very farthest ending and approach it like
        // a merging interval problem
        let mut last_index: std::collections::HashMap<char, usize> =
            std::collections::HashMap::new();
        for (idx, letter) in s.chars().enumerate() {
            last_index.insert(letter, idx);
        }
        let mut farthest_end = 0;
        let mut partition_size = 0;
        let mut result = Vec::new();
        for (idx, letter) in s.chars().enumerate() {
            let cur_last_appearance = *last_index.get(&letter).unwrap();
            // dbg!(cur_last_appearance);
            farthest_end = farthest_end.max(cur_last_appearance);
            partition_size += 1;
            // dbg!(farthest_end);
            if farthest_end == idx {
                result.push(partition_size);
                partition_size = 0;
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
        let s = "ababcbacadefegdehijhklij".to_string();
        let output = vec![9, 7, 8];
        let result = Solution::partition_labels(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "eccbbbbdec".to_string();
        let output = vec![10];
        let result = Solution::partition_labels(s);
        assert_eq!(result, output);
    }
}
