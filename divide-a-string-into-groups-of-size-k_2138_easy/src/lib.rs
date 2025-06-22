#[allow(unused)]
struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_chunk = String::new();
        let k = k as usize;

        for c in s.chars() {
            current_chunk.push(c);
            if current_chunk.len() == k {
                result.push(current_chunk);
                current_chunk = String::new();
            }
        }

        if !current_chunk.is_empty() {
            while current_chunk.len() < k {
                current_chunk.push(fill);
            }
            result.push(current_chunk);
        }

        result
    }
}
