struct Solution;

impl Solution {
    pub fn min_bit_flips_long_way(start: i32, goal: i32) -> i32 {
        let mut start_bytes = format!("{:b}", start).into_bytes();
        let mut goal_bytes = format!("{:b}", goal).into_bytes();

        while goal_bytes.len() < start_bytes.len() {
            goal_bytes.insert(0, 48);
        }

        while goal_bytes.len() > start_bytes.len() {
            start_bytes.insert(0, 48);
        }
        // dbg!(&start_bytes);
        // dbg!(&goal_bytes);

        let mut count = 0;
        for i in 0..start_bytes.len() {
            if start_bytes[i] != goal_bytes[i] {
                count += 1;
            }
        }
        count
    }

    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let remove_duplicates = start ^ goal;
        remove_duplicates.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let start = 10; // 1010
        let goal = 7; // 0111
        let output = 3;
        let result = Solution::min_bit_flips(start, goal);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let start = 10; // 1010
        let goal = 82; // 0111
        let output = 3;
        let result = Solution::min_bit_flips(start, goal);
        assert_eq!(result, output);
    }
}
