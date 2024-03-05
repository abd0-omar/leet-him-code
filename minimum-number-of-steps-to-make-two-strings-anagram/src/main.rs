fn main() {
    println!("Hello, world!");

    //     Example 1:
    //
    let s = "bab".to_string();
    let t = "aba".to_string();
    println!("{}", min_steps(s, t));
    // Output: 1
    // Explanation: Replace the first 'a' in t with b, t = "bba" which is anagram of s.
    // Example 2:
    //
    let s = "leetcode".to_string();
    let t = "practice".to_string();
    println!("{}", min_steps(s, t));
    // Output: 5
    // Explanation: Replace 'p', 'r', 'a', 'i' and 'c' from t with proper characters to make t anagram of s.
    // Example 3:
    //
    let s = "anagram".to_string();
    let t = "mangaar".to_string();
    println!("{}", min_steps(s, t));
    // Output: 0
    // Explanation: "anagram" and "mangaar" are anagrams.
}

pub fn min_steps(s: String, t: String) -> i32 {
    let x = &s[0..1];
    let s = s.as_bytes();
    let t = t.as_bytes();
    // let mut hm = std::collections::HashMap::new();
    let mut freq = [0; 26];
    for i in 0..s.len() {
        // let s_char_index = s[i];
        freq[(s[i] - b'a') as usize] += 1;
        // *hm.entry(s[i]).or_insert(0) += 1;
        // *hm.entry(t[i]).or_insert(0) -= 1;
        freq[(t[i] - b'a') as usize] -= 1;
    }

    // hm.values().filter(|&x| *x > 0).sum::<i32>()
    freq.iter().filter(|&x| *x > 0).sum()
}
