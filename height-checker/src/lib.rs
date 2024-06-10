pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    let mut count = 0;
    expected.sort_unstable();

    for i in 0..heights.len() {
        if heights[i] != expected[i] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heights = vec![1, 1, 4, 2, 1, 3];
        let output = 3;
        // Explanation:
        // heights:  [1,1,4,2,1,3]
        // expected: [1,1,1,2,3,4]
        // Indices 2, 4, and 5 do not match.
        let result = height_checker(heights);
        assert_eq!(result, output);
    }
}
