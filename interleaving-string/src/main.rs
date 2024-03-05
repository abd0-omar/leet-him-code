use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbcbcac".to_string();
    println!("{}", is_interleave(s1, s2, s3));
    // Output: true
    // Explanation: One way to obtain s3 is:
    // Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
    // Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
    // Since s3 can be obtained by interleaving s1 and s2, we return true.
    // Example 2:
    //
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbbaccc".to_string();
    println!("{}", is_interleave(s1, s2, s3));
    // Output: false

    let s1 = "a".to_string();
    let s2 = "b".to_string();
    let s3 = "a".to_string();
    println!("{}", is_interleave(s1, s2, s3));
    // Output: false
}

pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    if s1.is_empty() || s2.is_empty() {
        if s1 == s3 || s2 == s3 {
            return true;
        } else {
            return false;
        }
    }
    let mut memory = HashMap::new();
    _is_interleave(
        s1.as_bytes(),
        s2.as_bytes(),
        s3.as_bytes(),
        0,
        0,
        &mut memory,
    )
}

pub fn _is_interleave(
    s1: &[u8],
    s2: &[u8],
    s3: &[u8],
    idx1: usize,
    idx2: usize,
    memory: &mut HashMap<(usize, usize), bool>,
) -> bool {
    // just put all on hashset for now
    if idx1 == s1.len() && idx2 == s2.len() {
        return true;
    }

    if let Some(&ret) = memory.get(&(idx1, idx2)) {
        return ret;
    }

    let mut choice1 = false;

    //pick from a and leave from b
    // pick only if idx < s1.len()
    if idx1 < s1.len() && s1[idx1] == s3[idx1 + idx2] {
        choice1 = _is_interleave(s1, s2, s3, idx1 + 1, idx2, memory);
    }

    let mut choice2 = false;

    //pick from b and leave from a
    if idx2 < s2.len() && s2[idx2] == s3[idx1 + idx2] {
        choice2 = _is_interleave(s1, s2, s3, idx1, idx2 + 1, memory);
    }

    let result = choice1 || choice2;
    memory.insert((idx1, idx2), result);
    result
}
